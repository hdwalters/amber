use super::expr::Expr;
use crate::modules::types::Type;
use crate::utils::ParserMetadata;
use heraclitus_compiler::prelude::*;

pub mod is;
pub mod cast;

pub trait TypeOp: SyntaxModule<ParserMetadata> {
    fn set_left(&mut self, left: Expr);
    fn set_right(&mut self, right: Type);
    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult;
}
