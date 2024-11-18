use heraclitus_compiler::prelude::*;
use crate::docs::module::DocumentationModule;
use crate::modules::expression::expr::Expr;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CliArg {
    parser: Box<Option<Expr>>,
    option: Box<Option<Expr>>,
    default: Box<Option<Expr>>,
    help: Box<Option<Expr>>,
}

impl Typed for CliArg {
    fn get_type(&self) -> Type {
        (*self.default).as_ref()
            .map(|default| default.kind.clone())
            .unwrap_or(Type::Null)
    }
}

impl SyntaxModule<ParserMetadata> for CliArg {
    syntax_name!("Argument Invocation");

    fn new() -> Self {
        Self {
            parser: Box::new(None),
            option: Box::new(None),
            default: Box::new(None),
            help: Box::new(None),
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "arg")?;
        let mut parser = Expr::new();
        let mut option = Expr::new();
        let mut default = Expr::new();
        let mut help = Expr::new();
        token(meta, "(")?;
        syntax(meta, &mut parser)?;
        token(meta, ",")?;
        syntax(meta, &mut option)?;
        token(meta, ",")?;
        syntax(meta, &mut default)?;
        token(meta, ",")?;
        syntax(meta, &mut help)?;
        token(meta, ")")?;
        self.parser = Box::new(Some(parser));
        self.option = Box::new(Some(option));
        self.default = Box::new(Some(default));
        self.help = Box::new(Some(help));
        Ok(())
    }
}

impl TranslateModule for CliArg {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        todo!()
    }
}

impl DocumentationModule for CliArg {
    fn document(&self, _meta: &ParserMetadata) -> String {
        todo!()
    }
}
