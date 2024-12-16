use crate::modules::builtin::cli::parser::ParserImpl;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Payload {
    Parser(Rc<RefCell<ParserImpl>>),
}
