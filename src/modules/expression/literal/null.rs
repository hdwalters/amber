use crate::docs::module::DocumentationModule;
use crate::fragments;
use crate::modules::prelude::*;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct Null {}

impl Typed for Null {
    fn get_type(&self) -> Type {
        Type::Null
    }
}

impl SyntaxModule<ParserMetadata> for Null {
    syntax_name!("Null");

    fn new() -> Self {
        Null {}
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "null")?;
        Ok(())
    }
}

impl TranslateModule for Null {
    fn translate(&self, _meta: &mut TranslateMetadata) -> FragmentKind {
        fragments!("''")
    }
}

impl DocumentationModule for Null {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
