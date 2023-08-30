use crate::{ProgramSource, grammar};

#[salsa::tracked]
pub fn parse_program(db: &dyn crate::ParserDatabase, source: ProgramSource) {
    let parsed = grammar::ProgramParser::new().parse(db, &source.text(db));
    // original code: 
    // let program = parse_statements(db, source_program);
    // type_check_program(db, program);
}
