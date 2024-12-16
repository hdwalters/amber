use crate::docs::module::DocumentationModule;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct CliParser {
}

impl Typed for CliParser {
    fn get_type(&self) -> Type {
        todo!()
    }
}

impl SyntaxModule<ParserMetadata> for CliParser {
    syntax_name!("Parser Invocation");

    fn new() -> Self {
        Self { }
    }

    fn parse(&mut self, _meta: &mut ParserMetadata) -> SyntaxResult {
        todo!()
    }
}

impl TranslateModule for CliParser {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        todo!()
    }
}

impl DocumentationModule for CliParser {
    fn document(&self, _meta: &ParserMetadata) -> String {
        todo!()
    }
}
