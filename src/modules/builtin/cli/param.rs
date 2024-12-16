use crate::docs::module::DocumentationModule;
use crate::modules::builtin::cli::parser::ParserImpl;
use crate::modules::expression::expr::Expr;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct ParamImpl {
    option: String,
    default: Expr,
    help: String,
}

impl ParamImpl {
    fn new(option: String, default: Expr, help: String) -> Self {
        Self { option, default, help }
    }
}

#[derive(Debug, Clone)]
pub struct ParamCli {
    param: Option<Rc<ParamImpl>>,
}

impl Typed for ParamCli {
    fn get_type(&self) -> Type {
        self.param.as_ref()
            .map(|param| param.default.kind.clone())
            .unwrap_or(Type::Null)
    }
}

impl SyntaxModule<ParserMetadata> for ParamCli {
    syntax_name!("Argument Invocation");

    fn new() -> Self {
        Self { param: None }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "param")?;
        let mut parser = Expr::new();
        let mut option = Expr::new();
        let mut default = Expr::new();
        let mut help = Expr::new();
        token(meta, "(")?;
        let parser_tok = meta.get_current_token();
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
        let parser = match ParserImpl::find_parser(meta, &parser) {
            Some(parser) => parser,
            None => return error!(meta, parser_tok, "Expected parser object"),
        };
        let option = match option.get_literal_text() {
            Some(option) => option,
            None => return error!(meta, option_tok, "Expected literal string"),
        };
        let help = match help.get_literal_text() {
            Some(help) => help,
            None => return error!(meta, help_tok, "Expected literal string"),
        };
        let param = Rc::new(ParamImpl::new(option, default, help));
        parser.borrow_mut().add_param(Rc::clone(&param));
        self.param = Some(param);
        Ok(())
    }
}

impl TranslateModule for ParamCli {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        self.param.as_ref()
            .map(|param| param.default.translate(meta))
            .unwrap_or_default()
    }
}

impl DocumentationModule for ParamCli {
    fn document(&self, _meta: &ParserMetadata) -> String {
        String::from("")
    }
}
