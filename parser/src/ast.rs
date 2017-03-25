//! Syntactic elements of assembly.

/// A unit that can stand by itself in a program.
#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    /// Label declaration.
    Label(Label),
    /// Processor operation.
    Opcode(Opcode),
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
pub struct Opcode {
    pub name: String,
    pub mode: OpcodeMode,
    pub value: Expression,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpcodeMode {
    Implied, // no argument
    Immediate, // #$
    Address, // $
    XAddress, // $,x
    YAddress, // $,y
    StackAddress, // $,s
    Indirect, // ($)
    XIndirect, // ($,x)
    IndirectY, // ($),y
    StackIndirectY, // ($,s),y
    LongIndirect, // [$]
    LongIndirectY, // [$],y
    Move { first_bank: Expression }, // $,$
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
pub struct Number {
    pub value: u32,
    pub width: NumberWidth,
}

/// A byte width of numeric literal.
///
/// Some numeric literals have their own suggested byte width, mostly used
/// for purpose of determining the size of immediate instruction. 65c816 has
/// two immediate instructions, sharing the same opcode depending on CPU
/// mode, and using a wrong one will likely lead to a crash. The assembler
/// doesn't try to guess the size, other than a very specific case of
/// hexadecimal or binary literal that is exactly one or two bytes. However,
/// because that special case does exist, it needs to be in AST.
///
/// For instance, the following program uses two different versions of the
/// same opcode (A9). The distinction between those is at runtime, by checking
/// processor flags.
///
/// ```asm
/// LDA #$10   ; Interpreted as one byte literal,  A9 10
/// LDA #$1000 ; Interpreted as two bytes literal, A9 00 10
/// ```
///
/// This is useless outside of immediate instructions that work on accumulator
/// or indexes where the number value comes directly from byte literal or
/// variable storing such (without any operations done on it).
#[derive(Debug, Eq, PartialEq)]
pub enum NumberWidth {
    None,
    OneByte,
    TwoBytes,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Number(Number),
    Variable(Label),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Call(VariableName, Vec<Expression>),
}
