//! Syntactic elements of assembly.

/// A unit that can stand by itself in a program.
#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    /// Label declaration.
    Label(Label),
    /// Processor operation.
    Opcode(Opcode),
    /// Assembly keyword.
    Keyword(Keyword),
    /// Group of if blocks, possibly with else if conditions.
    If(Vec<Condition>),
    /// Assignment of `Expression` to `VariableName`.
    Assignment(VariableName, Expression),
}

/// An unique name of an identifier in a program.
///
/// Most of time, a `Label` is used when a reference to a value is needed,
/// however variable names are in grammar to support those cases where
/// a relative label reference is not acceptable, in particular assignments.
#[derive(Debug, Eq, PartialEq)]
pub struct VariableName(pub String);

/// A reference to a location in assembly.
///
/// It can be named or relative. Relative location is a signed integer
/// whose level of depth is determined by a number, negative integers
/// mean backward references, while positive numbers mean forward
/// references.
#[derive(Debug, Eq, PartialEq)]
pub enum Label {
    Named(VariableName),
    Relative(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Opcode {

}

/// Assembler keyword.
#[derive(Debug, Eq, PartialEq)]
pub enum Keyword {
    /// A keyword that changes the position the code is written to.
    Org(Expression),
}

/// A single "if" block with predicate and statements.
///
/// This is usually used in a `Vec`, and represents a single predicate along
/// with statements to run if it is met.
#[derive(Debug, Eq, PartialEq)]
pub struct Condition {
    pub predicate: Option<Expression>,
    pub statements: Vec<Statement>,
}

/// An operator that takes two arguments
///
/// Those operators map to mathematical operators on numbers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    /// Addition (`+`).
    Add,
    /// Subtraction (`-`).
    Sub,
    /// Multiplication (`*`).
    Mul,
    /// Division (`/`).
    Div,

    /// Shift left (`<<`).
    Shl,
    /// Shift right (`>>`).
    Shr,
    /// Bitwise xor (`^`).
    Xor,
    /// Bitwise and (`&`).
    And,
    /// Bitwise or (`|`).
    Or,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Number { value: u32, width: Option<usize> },
    Variable(Label),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Call(VariableName, Vec<Expression>),
}
