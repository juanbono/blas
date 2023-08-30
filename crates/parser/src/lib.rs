mod ast;
mod db;
mod parse;

pub use ast::{Program, ProgramId, ProgramSource};
pub use {db::Database, parse::parse_program};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP


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

// This is some required salsa boilerplate.
pub trait ParserDatabase: salsa::DbWithJar<Jar> {}
impl<DB> ParserDatabase for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
