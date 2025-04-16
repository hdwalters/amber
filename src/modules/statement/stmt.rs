use super::comment::Comment;
use super::comment_doc::CommentDoc;
use crate::docs::module::DocumentationModule;
use crate::handle_types;
use crate::modules::builtin::cd::Cd;
use crate::modules::builtin::echo::Echo;
use crate::modules::builtin::exit::Exit;
use crate::modules::builtin::mv::Mv;
use crate::modules::command::modifier::CommandModifier;
use crate::modules::condition::ifchain::IfChain;
use crate::modules::condition::ifcond::IfCondition;
use crate::modules::expression::expr::{Expr, ExprType};
use crate::modules::function::declaration::FunctionDeclaration;
use crate::modules::function::fail::Fail;
use crate::modules::function::ret::Return;
use crate::modules::imports::import::Import;
use crate::modules::loops::break_stmt::Break;
use crate::modules::loops::continue_stmt::Continue;
use crate::modules::loops::infinite_loop::InfiniteLoop;
use crate::modules::loops::iter_loop::IterLoop;
use crate::modules::main::Main;
use crate::modules::prelude::*;
use crate::modules::shorthand::add::ShorthandAdd;
use crate::modules::shorthand::div::ShorthandDiv;
use crate::modules::shorthand::modulo::ShorthandModulo;
use crate::modules::shorthand::mul::ShorthandMul;
use crate::modules::shorthand::sub::ShorthandSub;
use crate::modules::variable::init::VariableInit;
use crate::modules::variable::set::VariableSet;
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub enum StatementType {
    Expr(Expr),
    VariableInit(VariableInit),
    VariableSet(VariableSet),
    IfCondition(IfCondition),
    IfChain(IfChain),
    ShorthandAdd(ShorthandAdd),
    ShorthandSub(ShorthandSub),
    ShorthandMul(ShorthandMul),
    ShorthandDiv(ShorthandDiv),
    ShorthandModulo(ShorthandModulo),
    InfiniteLoop(InfiniteLoop),
    IterLoop(IterLoop),
    Break(Break),
    Continue(Continue),
    FunctionDeclaration(FunctionDeclaration),
    Return(Return),
    Fail(Fail),
    Import(Import),
    Main(Main),
    Cd(Cd),
    Echo(Echo),
    Mv(Mv),
    Exit(Exit),
    CommandModifier(CommandModifier),
    Comment(Comment),
    CommentDoc(CommentDoc),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub value: Option<StatementType>
}

impl Statement {
    handle_types!(StatementType, [
        // Imports
        Import,
        // Functions
        FunctionDeclaration, Main, Return, Fail,
        // Loops
        InfiniteLoop, IterLoop, Break, Continue,
        // Conditions
        IfChain, IfCondition,
        // Variables
        VariableInit, VariableSet,
        // Short hand
        ShorthandAdd, ShorthandSub,
        ShorthandMul, ShorthandDiv,
        ShorthandModulo,
        // Command
        CommandModifier, Echo, Mv, Cd, Exit,
        // Comment doc
        CommentDoc, Comment,
        // Expression
        Expr
    ]);

    // Get result out of the provided module and save it in the internal state
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: impl Fn(S) -> StatementType) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        match syntax(meta, &mut module) {
            Ok(()) => {
                self.value = Some(cb(module));
                Ok(())
            }
            Err(details) => Err(details)
        }
    }

    pub fn get_docs_item_name(&self) -> Option<String> {
        match &self.value {
            Some(StatementType::FunctionDeclaration(inner)) => Some(inner.name.clone()),
            _ => None,
        }
    }
}

impl SyntaxModule<ParserMetadata> for Statement {
    syntax_name!("Statement");

    fn new() -> Self {
        Statement {
            value: None
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let mut error = None;
        let statements = self.get_modules();
        for statement in statements {
            // Try to parse the statement
            match self.parse_match(meta, statement) {
                Ok(()) => return Ok(()),
                Err(failure) => {
                    match failure {
                        Failure::Loud(err) => return Err(Failure::Loud(err)),
                        Failure::Quiet(err) => error = Some(err)
                    }
                }
            }
        }
        Err(Failure::Quiet(error.unwrap()))
    }
}

impl TranslateModule for Statement {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        // Translate the staxtement
        let statement = self.value.as_ref().unwrap();
        // This is a workaround that handles $(...) which cannot be used as a statement
        match statement {
            StatementType::Expr(expr) => {
                match &expr.value {
                    Some(ExprType::Command(cmd)) => {
                        cmd.translate_command_statement(meta)
                    },
                    _ => {
                        self.translate_match(meta, statement);
                        FragmentKind::Empty
                    }
                }
            },
            _ => {
                self.translate_match(meta, statement)
            }
        }
    }
}

impl DocumentationModule for Statement {
    fn document(&self, meta: &ParserMetadata) -> String {
        // Document the statement
        let documented = self.document_match(meta, self.value.as_ref().unwrap());
        documented
    }
}
