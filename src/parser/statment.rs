use color_eyre::eyre::Result;

use crate::parser::{
    Parser,
    ast::{ASTStatment, ASTStatmentKind},
};

impl Parser {
    pub fn parse_statment(&mut self) -> Result<ASTStatment> {
        let expr = self.parse_expression()?;
        Ok(ASTStatment {
            span: expr.span.clone(),
            kind: ASTStatmentKind::Expression(expr),
        })
    }
}
