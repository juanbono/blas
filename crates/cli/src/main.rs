use core::RootDatabase;

fn main() {
    let db = RootDatabase::default();
    let program = r#"
    program HolaMundo 
    end"#
        .to_string();

    let compiled_program = db.compile_string(program).unwrap();
    dbg!(compiled_program);
    dbg!(compiled_program.program_id(&db).text(&db));
}
