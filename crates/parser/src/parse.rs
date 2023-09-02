use crate::{grammar, Program, ProgramSource};

#[salsa::tracked]
pub fn parse_program(db: &dyn crate::ParserDatabase, source: ProgramSource) -> Result<Program, ()> {
    grammar::ProgramParser::new()
        .parse(db, source.text(db))
        .map_err(|_| ())
}
