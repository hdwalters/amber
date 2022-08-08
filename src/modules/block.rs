use heraclitus_compiler::prelude::*;
use crate::utils::{metadata::ParserMetadata, error::get_error_logger};
use super::statement::statement::Statement;

#[derive(Debug)]
pub struct Block {
    statements: Vec<Statement>
}

impl Block {
    fn error(&mut self, meta: &mut ParserMetadata, details: ErrorDetails) {
        get_error_logger(meta, details)
            .attach_message("Undefined syntax")
            .show()
            .exit()
    }
}

impl SyntaxModule<ParserMetadata> for Block {
    syntax_name!("Block");

    fn new() -> Self {
        Block {
            statements: vec![]
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        meta.var_mem.push_scope();
        loop {
            if let None = meta.get_token_at(meta.get_index()) {
                break;
            }
            let mut statemant = Statement::new();
            if let Err(details) = statemant.parse(meta) {
                self.error(meta, details);
            }
            self.statements.push(statemant);
        }
        meta.var_mem.pop_scope();
        Ok(())
    }
}