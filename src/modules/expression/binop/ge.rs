use super::BinOp;
use crate::modules::expression::expr::Expr;
use crate::modules::prelude::*;
use crate::modules::types::{Type, Typed};
use crate::translate::compute::{translate_computation, ArithOp};
use crate::{error_type_match, handle_binop};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct Ge {
    left: Box<Expr>,
    right: Box<Expr>
}

impl Typed for Ge {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl BinOp for Ge {
    fn set_left(&mut self, left: Expr) {
        self.left = Box::new(left);
    }

    fn set_right(&mut self, right: Expr) {
        self.right = Box::new(right);
    }

    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, ">=")?;
        Ok(())
    }
}

impl SyntaxModule<ParserMetadata> for Ge {
    syntax_name!("Ge");

    fn new() -> Self {
        Ge {
            left: Box::new(Expr::new()),
            right: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        handle_binop!(meta, "compare", self.left, self.right, [Num])?;
        Ok(())
    }
}

impl TranslateModule for Ge {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        let left = self.left.translate(meta);
        let right = self.right.translate(meta);
        translate_computation(meta, ArithOp::Ge, Some(left), Some(right))
    }
}

impl DocumentationModule for Ge {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
