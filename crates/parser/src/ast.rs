use derive_new::new;

/// Represents the program source code. This is the input of the compiler.
/// From this, all information is derived.
#[salsa::input]
pub struct ProgramSource {
    /// the program source code.
    #[return_ref]
    pub text: String,
}

#[salsa::interned]
pub struct ProgramId {
    pub text: String,
}

#[salsa::interned]
pub struct FunctionId {
    #[return_ref]
    pub text: String,
}

#[salsa::tracked]
pub struct Program {
    #[return_ref]
    pub program_id: ProgramId,
    // TODO: implement statements
    // #[return_ref]
    // pub statements: Vec<Statement>,
}

#[derive(Eq, PartialEq, Debug, Hash, new)]
pub struct Statement;
