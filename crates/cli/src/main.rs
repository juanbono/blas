use core::RootDatabase;

use clap::Parser;

/// Arguments to the compiler CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of file to compile
    #[arg(short, long)]
    file: String,
}

fn main() {
    // parse command line arguments
    let args = Args::parse();

    // read the program from file
    let program = std::fs::read_to_string(args.file).unwrap();

    // create a new database where to store the program
    let db = RootDatabase::default();

    // compile the program
    let compiled_program = db.compile_string(program).unwrap();

    dbg!(compiled_program);
    dbg!(compiled_program.statements(&db));
    dbg!(compiled_program.program_id(&db).text(&db));
}
