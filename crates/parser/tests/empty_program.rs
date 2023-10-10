use parser::{ExpressionData, StatementData};

#[test]
fn empty_program_test() {
    let db = parser::Database::default();
    let program = "program HolaMundo end".to_string();
    let source = parser::ProgramSource::new(&db, program);
    let parsed_program = parser::parse_program(&db, source);

    // check that there was no error while parsing.
    dbg!(&parsed_program);
    assert!(parsed_program.is_ok());

    // read the parsed program id text from the database.
    let parsed_program_id = parsed_program.unwrap().program_id(&db).text(&db);
    // and check that it is the same as the one in our program.
    assert_eq!("HolaMundo", parsed_program_id);
}

#[test]
fn return_statement_test() {
    let db = parser::Database::default();
    let program = "program Test\nbegin\nreturn 42;\nend\nend".to_string();
    let source = parser::ProgramSource::new(&db, program);
    let parsed_program = parser::parse_program(&db, source);

    // check that there was no error while parsing.
    assert!(parsed_program.is_ok());

    // get the first statement from the main function.
    let statement = parsed_program
        .unwrap()
        .statements(&db)
        .into_iter()
        .next()
        .unwrap();

    // check that the statement is a return statement with the correct value.
    match &statement.data {
        StatementData::Return(expression) => match expression.data {
            ExpressionData::Number(number) => {
                assert_eq!(number, 42);
            }
        },
    }
}
