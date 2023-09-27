mod compile;
mod db;

pub use db::RootDatabase;

/// The Jar combines all the features provided by the salsa database.
/// Every tracked function, interned value, query and input must be listed here.
#[salsa::jar(db = CompilerDatabase)]
pub struct Jar(crate::compile::parse_program);

// TODO: Document this trait.
pub trait CompilerDatabase: salsa::DbWithJar<Jar> + salsa::DbWithJar<parser::Jar> {}

// blanket implementation for every type that implements DbWithJar<Jar>.
// This will allow the db::Database to implement ParserDatabase without a
// concrete implemetation.
impl<DB> CompilerDatabase for DB where
    DB: ?Sized + salsa::DbWithJar<Jar> + salsa::DbWithJar<parser::Jar>
{
}
