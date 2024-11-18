use crate::docs::module::DocumentationModule;
use crate::modules::expression::expr::Expr;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
struct ArgImpl {
    option: String,
    default: Expr,
    help: String,
}

impl ArgImpl {
    fn new(option: String, default: Expr, help: String) -> Self {
        Self { option, default, help }
    }
}

#[derive(Debug, Clone)]
pub struct CliArg {
    arg: Option<Rc<ArgImpl>>,
}

impl Typed for CliArg {
    fn get_type(&self) -> Type {
        self.arg.as_ref()
            .map(|arg| arg.default.kind.clone())
            .unwrap_or(Type::Null)
    }
}

impl SyntaxModule<ParserMetadata> for CliArg {
    syntax_name!("Argument Invocation");

    fn new() -> Self {
        Self { arg: None }
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
        let option_tok = meta.get_current_token();
        syntax(meta, &mut option)?;
        token(meta, ",")?;
        syntax(meta, &mut default)?;
        token(meta, ",")?;
        let help_tok = meta.get_current_token();
        syntax(meta, &mut help)?;
        token(meta, ")")?;
        let option = match option.get_text_if_literal() {
            Some(option) => option,
            None => return error!(meta, option_tok, "Expected literal string"),
        };
        let help = match help.get_text_if_literal() {
            Some(help) => help,
            None => return error!(meta, help_tok, "Expected literal string"),
        };
        let arg = ArgImpl::new(option, default, help);
        self.arg = Some(Rc::new(arg));
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
