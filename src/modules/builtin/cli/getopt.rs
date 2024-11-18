use crate::docs::module::DocumentationModule;
use crate::modules::expression::expr::Expr;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CliGetopt {
    parser: Box<Option<Expr>>,
    args: Box<Option<Expr>>,
}

impl Typed for CliGetopt {
    fn get_type(&self) -> Type {
        Type::Null
    }
}

impl SyntaxModule<ParserMetadata> for CliGetopt {
    syntax_name!("Getopt Invocation");

    fn new() -> Self {
        Self {
            parser: Box::new(None),
            args: Box::new(None),
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "parse")?;
        let mut parser = Expr::new();
        let mut args = Expr::new();
        token(meta, "(")?;
        syntax(meta, &mut parser)?;
        token(meta, ",")?;
        syntax(meta, &mut args)?;
        token(meta, ")")?;
        Ok(())
    }
}

impl TranslateModule for CliGetopt {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        todo!()
    }
}

impl DocumentationModule for CliGetopt {
    fn document(&self, _meta: &ParserMetadata) -> String {
        todo!()
    }
}
