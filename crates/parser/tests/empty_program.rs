#[test]
fn empty_program_test() {
    let db = parser::Database::default();
    let program = "program HolaMundo end".to_string();
    let source = parser::ProgramSource::new(&db, program);
    let parsed_program = parser::parse_program(&db, source);

    // check that there was no error while parsing.
    assert!(parsed_program.is_ok());

    // read the parsed program id text from the database.
    let parsed_program_id = parsed_program.unwrap().program_id(&db).text(&db);
    // and check that it is the same as the one in our program.
    assert_eq!("HolaMundo", parsed_program_id);
}
