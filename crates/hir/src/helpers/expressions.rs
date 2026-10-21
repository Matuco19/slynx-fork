use crate::{
    HIRError, Owned, Result, SlynxHir, SymbolPointer,
    model::{HirExpression, HirExpressionKind},
    term::{PrimitiveType, Term, TermId, TermNode},
};
use common::{Operator, Span, Spanned, pool::PoolId};
use module_loader::FileId;

impl<'a> SlynxHir<'a> {
    /// Creates a string literal expression owned by `owner`.
    pub(crate) fn create_strliteral_expression(
        &self,
        owner: FileId,
        s: SymbolPointer,
    ) -> HirExpression {
        HirExpression {
            ty: Owned::new(owner, self.types.create_type(Term::string_type())),
            kind: HirExpressionKind::StringLiteral(s),
        }
    }

    /// Creates an int expression that must be inferred, owned by `owner`.
    pub(crate) fn create_int_expression(&self, owner: FileId, i: i32, ty: TermId) -> HirExpression {
        HirExpression {
            kind: HirExpressionKind::Int(i),
            ty: Owned::new(owner, ty),
        }
    }

    /// Creates a float expression owned by `owner`.
    pub(crate) fn create_float_expression(
        &self,
        owner: FileId,
        float: f32,
        ty: TermId,
        span: Span,
    ) -> Result<HirExpression> {
        match self.view(ty).raw().node() {
            TermNode::Primitive(PrimitiveType::Float32 | PrimitiveType::Float64) => {}
            _ => return Err(HIRError::expected_float_type(owner, ty, span)),
        }
        Ok(HirExpression {
            kind: HirExpressionKind::Float(float.into()),
            ty: Owned::new(owner, ty),
        })
    }
    /// Creates a binary expression owned by `owner`.
    pub(crate) fn create_binary_expression(
        &self,
        owner: FileId,
        left: Spanned<PoolId<HirExpression>>,
        right: Spanned<PoolId<HirExpression>>,
        operator: Operator,
        ty: TermId,
    ) -> HirExpression {
        HirExpression {
            kind: HirExpressionKind::Binary {
                lhs: left,
                op: operator,
                rhs: right,
            },
            ty: Owned::new(owner, ty),
        }
    }
}
