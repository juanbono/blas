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

    #[return_ref]
    pub statements: Vec<Statement>,
}

#[derive(Eq, PartialEq, Debug, Hash, new)]
pub struct Statement {
    pub data: StatementData,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum StatementData {
    /// Defines `fn <name>(<args>) = <body>`
    // Function(Function),
    /// Defines `writeln(<expr>)`
    // WriteLn(Expression),

    /// Returns an expression
    Return(Expression),
}

#[derive(Eq, PartialEq, Debug, Hash, new)]
pub struct Expression {
    // pub span: Span,
    pub data: ExpressionData,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum ExpressionData {
    // Op(Box<Expression>, Op, Box<Expression>),
    Number(u64),
    // Variable(VariableId),
    // Call(FunctionId, Vec<Expression>),
}
