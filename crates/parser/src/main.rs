use salsa::ParallelDatabase;



#[test]
fn calculator1() {
    let db = parser::Database::default();
    let source = parser::ProgramSource::new(&db, String::from("program HolaMundo end"));
    parser::parse_program(&db, source);
    // dbg!(&parsed_ast);
    
    // assert!(parsed_ast.is_ok());
    // assert!(grammar::TermParser::new().parse(&mut db, "(22)").is_ok());
    // assert!(grammar::TermParser::new().parse(&mut db, "((((22))))").is_ok());
    // assert!(grammar::TermParser::new().parse(&mut db, "((22)").is_err());
}

fn main() {
    println!("Hello, world!");
}
