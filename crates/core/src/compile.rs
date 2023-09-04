use parser::ParseError;
use salsa::DbWithJar;

#[salsa::tracked]
pub fn compile_program(
    db: &dyn crate::CompilerDatabase,
    source: parser::ProgramSource,
) -> Result<parser::Program, ParseError> {
    let parser_db = <dyn crate::CompilerDatabase as DbWithJar<parser::Jar>>::as_jar_db(db);
    parser::parse_program(parser_db, source)
}
