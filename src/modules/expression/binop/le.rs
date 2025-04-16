use super::BinOp;
use crate::modules::expression::expr::Expr;
use crate::modules::prelude::*;
use crate::modules::types::{Type, Typed};
use crate::translate::compute::{translate_computation, ArithOp};
use crate::{error_type_match, handle_binop};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct Le {
    left: Box<Expr>,
    right: Box<Expr>
}

impl Typed for Le {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl BinOp for Le {
    fn set_left(&mut self, left: Expr) {
        self.left = Box::new(left);
    }

    fn set_right(&mut self, right: Expr) {
        self.right = Box::new(right);
    }

    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "<=")?;
        Ok(())
    }
}

impl SyntaxModule<ParserMetadata> for Le {
    syntax_name!("Le");

    fn new() -> Self {
        Le {
            left: Box::new(Expr::new()),
            right: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        handle_binop!(meta, "compare", self.left, self.right, [Num])?;
        Ok(())
    }
}

impl TranslateModule for Le {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        let left = self.left.translate(meta);
        let right = self.right.translate(meta);
        translate_computation(meta, ArithOp::Le, Some(left), Some(right))
    }
}

impl DocumentationModule for Le {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
