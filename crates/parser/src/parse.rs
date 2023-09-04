use crate::{grammar, ParseError, Program, ProgramSource};

#[salsa::tracked]
pub fn parse_program(
    db: &dyn crate::ParserDatabase,
    source: ProgramSource,
) -> Result<Program, ParseError> {
    grammar::ProgramParser::new()
        .parse(db, source.text(db))
        .map_err(|_| ParseError)
}
