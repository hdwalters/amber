use super::super::expr::Expr;
use super::UnOp;
use crate::docs::module::DocumentationModule;
use crate::error_type_match;
use crate::modules::prelude::FragmentKind;
use crate::modules::types::{Type, Typed};
use crate::translate::compute::{translate_computation, ArithOp};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::ParserMetadata;
use crate::utils::TranslateMetadata;
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct Not {
    expr: Box<Expr>
}

impl Typed for Not {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl UnOp for Not {
    fn set_expr(&mut self, expr: Expr) {
        self.expr = Box::new(expr);
    }

    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "not")?;
        Ok(())
    }
}

impl SyntaxModule<ParserMetadata> for Not {
    syntax_name!("Not");

    fn new() -> Self {
        Not {
            expr: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        if !matches!(self.expr.get_type(), Type::Bool) {
            let msg = self.expr.get_error_message(meta);
            return error_type_match!(meta, msg, "logically negate", (self.expr), [Bool])
        }
        Ok(())
    }
}

impl TranslateModule for Not {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        let expr = self.expr.translate(meta);
        translate_computation(meta, ArithOp::Not, None, Some(expr))
    }
}

impl DocumentationModule for Not {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
