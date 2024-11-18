use crate::docs::module::DocumentationModule;
use crate::modules::builtin::cli::arg::CliArg;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CliParser {
    args: Vec<CliArg>,
}

impl Typed for CliParser {
    fn get_type(&self) -> Type {
        Type::Null
    }
}

impl SyntaxModule<ParserMetadata> for CliParser {
    syntax_name!("Parser Invocation");

    fn new() -> Self {
        Self { args: Vec::new() }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "parser")?;
        token(meta, "(")?;
        token(meta, ")")?;
        Ok(())
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
