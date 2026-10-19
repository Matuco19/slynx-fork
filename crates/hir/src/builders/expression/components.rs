use common::{Span, Spanned, pool::PoolId};
use module_loader::{ASTType, ASTTypeKind};
use slynx_parser::{ComponentExpression, ComponentMemberValue, TypeContext};

use crate::{
    HIRError, HirComponentExpression, PropertyExpression, Result,
    builders::HirQueueBuilder,
    context::HirSymbol,
    generics::GenericTypeArguments,
    term::{TermId, TermNode},
};

use super::{ExpressionBuilder, ExpressionDescriptor};

pub struct ComponentExpressionDescriptor<'a> {
    pub component: &'a ComponentExpression,
    pub span: Span,
    ///The type expected at this position, if any. A bare component literal has
    ///no explicit type arguments of its own, so this is where the concrete
    ///arguments of an expected `Box<int>` come from before inference fills in
    ///whatever the properties themselves can determine.
    pub expected: Option<TermId>,
    pub context: &'a TypeContext<'a>,
}

impl ExpressionBuilder {
    pub(crate) fn build_component_expression(
        &mut self,
        queue: &HirQueueBuilder,
        ComponentExpressionDescriptor {
            component,
            span,
            expected,
            context,
        }: ComponentExpressionDescriptor,
    ) -> Result<Spanned<PoolId<HirComponentExpression>>> {
        let name = queue.get_plain_type(component.name).identifier;
        let lowered = if let Some(self_type) = self.self_type {
            queue.lowerer.lower_type_with_self(
                queue,
                self.file(),
                component.name,
                context,
                self_type,
            )?
        } else {
            queue
                .lowerer
                .lower_type(queue, self.file(), component.name, context)?
        };
        let ty = lowered.term;
        let ty_view = queue.hir.view(lowered.term);
        let Some(comp_view) = ty_view.nominal().is_component() else {
            return Err(HIRError::not_a_component(self.file(), lowered.term, span));
        };
        let concrete_name = queue.hir.types.get_component_name(comp_view.data);
        let concrete_owner = queue.hir.types.get_component_owner(comp_view.data);
        let symbol = HirSymbol {
            owner: concrete_owner,
            term: concrete_name,
        };
        if queue.hir.find_component_by_symbol(symbol).is_none() {
            if let Some(ASTType {
                owner,
                content: ASTTypeKind::Component(comp),
            }) = queue
                .lowerer
                .lookup
                .find_type(concrete_owner, concrete_name)
            {
                let comp = queue.modules.get_entry(owner).component().get(comp);
                queue.enqueue_component(comp, owner)?;
            } else {
                return Err(HIRError::component_not_found(self.file(), name, span));
            }
        }
        let arity = queue
            .hir
            .find_component_by_symbol(symbol)
            .map(|decl| queue.hir.get_component(decl).generics.len())
            .unwrap_or(0);

        // Explicit type arguments can come from one of two places: a literal
        // that spells them out (`Box<int> { .. }`, whose lowered type is an
        // `Apply`), or the type expected at this position (`let b: Box<int> =
        // Box { .. }`). Both are recovered before inference so explicit
        // arguments always win over the ones we infer from property values.
        let mut generics = GenericTypeArguments::new();
        generics.reserve(arity);
        let explicit: &[TermId] = match ty_view.raw().node() {
            TermNode::Apply { args, .. } => args.as_slice(),
            _ => match expected {
                Some(expected)
                    if queue
                        .hir
                        .view(expected)
                        .nominal()
                        .is_component()
                        .map(|c| c.data())
                        == Some(comp_view.data())
                        && let TermNode::Apply { args, .. } =
                            queue.hir.view(expected).raw().node() =>
                {
                    args.as_slice()
                }
                _ => &[],
            },
        };
        generics.merge_explicit(explicit);

        let mut properties = Vec::new();
        let mut children = Vec::new();
        for value in &component.values {
            match value {
                ComponentMemberValue::Assign { prop_name, rhs }
                    if let Some(position) = comp_view
                        .props()
                        .iter()
                        .position(|prop| prop.name == *prop_name) =>
                {
                    let declared = comp_view.props()[position].ty;
                    // A parameter used directly as the property's type is the
                    // one place inference can read a concrete argument from:
                    // whatever the value expression turns out to be is the
                    // argument. Property types that merely *mention* the
                    // parameter (e.g. `Vec<T>`) are substituted but cannot
                    // contribute, so those still need an explicit argument.
                    let expected_ty = queue
                        .hir
                        .view(declared)
                        .raw()
                        .is_var_type()
                        .and_then(|var| generics.get(var.index as usize));
                    let expr = self.build_expression(
                        queue,
                        ExpressionDescriptor {
                            target: *rhs,
                            expected: expected_ty,
                            context,
                        },
                    )?;
                    if let Some(var) = queue.hir.view(declared).raw().is_var_type() {
                        generics.try_set(var.index as usize, queue.hir.view(expr.data).ty().term);
                    }
                    properties.push(PropertyExpression::new(position, expr));
                }
                ComponentMemberValue::Assign { prop_name, .. } => {
                    // couldnt find any member with name `prop_name`
                    return Err(HIRError::property_unrecognized(
                        self.file(),
                        ty,
                        vec![*prop_name],
                        span,
                    ));
                }
                ComponentMemberValue::Child(child) => {
                    let child_expr = self.build_component_expression(
                        queue,
                        ComponentExpressionDescriptor {
                            component: child,
                            span,
                            expected: None,
                            context,
                        },
                    )?;
                    children.push(child_expr);
                }
                ComponentMemberValue::NamedChild { name, child } => {
                    if let Some(position) = comp_view
                        .children()
                        .iter()
                        .position(|child| matches!(child.name, Some(cname) if cname == *name))
                    {
                        let expected = queue
                            .hir
                            .view(comp_view.children()[position].ty)
                            .raw()
                            .is_var_type()
                            .and_then(|var| generics.get(var.index as usize));
                        let child_expr = self.build_component_expression(
                            queue,
                            ComponentExpressionDescriptor {
                                component: child,
                                span,
                                expected,
                                context,
                            },
                        )?;
                        children.push(child_expr);
                    }
                }
            }
        }

        // A generic component is only usable once every parameter has a
        // concrete argument; anything still unresolved cannot be specialized
        // and must be reported rather than silently left as a bare `Box` that
        // codegen cannot lower.
        if arity > 0 && (0..arity).any(|index| !generics.is_resolved(index)) {
            return Err(HIRError::couldnt_infer(self.file(), span));
        }
        let concrete = if arity > 0 {
            generics.finish_ref(queue.hir, lowered.term)
        } else {
            lowered.term
        };

        let id = queue
            .hir
            .store
            .insert_component_expression(HirComponentExpression {
                name: concrete,
                properties,
                children,
            });
        Ok(span.make_spanned(id))
    }
}
