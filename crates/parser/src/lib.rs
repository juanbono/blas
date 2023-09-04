mod ast;
mod db;
mod parse;

use std::fmt::Display;

pub use ast::{
    Expression, ExpressionData, Program, ProgramId, ProgramSource, Statement, StatementData,
};
use lalrpop_util::lalrpop_mod;
use thiserror::Error;
pub use {db::Database, parse::parse_program};

// generate LALRPOP grammar module.
lalrpop_mod!(pub grammar);

/// The Jar combines all the features provided by the salsa database.
/// Every tracked function, interned value, query and input must be listed here.
#[salsa::jar(db = ParserDatabase)]
pub struct Jar(
    crate::parse::parse_program,
    crate::ast::ProgramSource,
    crate::ast::ProgramId,
    crate::ast::FunctionId,
    crate::ast::Program,
);

// TODO: Document this trait.
pub trait ParserDatabase: salsa::DbWithJar<Jar> {}

// blanket implementation for every type that implements DbWithJar<Jar>.
// This will allow the db::Database to implement ParserDatabase without a
// concrete implemetation.
impl<DB> ParserDatabase for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing program")
    }
}
