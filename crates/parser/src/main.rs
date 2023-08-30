fn main() {
    println!("Hello, world!");
}

#[test]
fn empty_program_test() {
    let db = parser::Database::default();
    let source = parser::ProgramSource::new(&db, String::from("program HolaMundo end"));
    let parsed_program = parser::parse_program(&db, source);

    assert!(parsed_program.is_ok());
    // read the program id text from the database.
    let program_id = parsed_program.unwrap().program_id(&db).text(&db);
    assert_eq!("HolaMundo", program_id);
}