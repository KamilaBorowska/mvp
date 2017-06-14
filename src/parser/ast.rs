//! Syntactic elements of assembly.

/// A unit that can stand by itself in a program.
#[derive(Debug, Eq, PartialEq)]
pub enum Statement<'a> {
    /// Label declaration.
    Label(Label<'a>),
    /// Processor operation.
    Opcode(Opcode<'a>),
    /// Group of if blocks, possibly with else if conditions.
    If(Vec<Condition<'a>>),
    /// Assignment of `Expression` to `VariableName`.
    Assignment(VariableName<'a>, Expression<'a>),
}

/// An unique name of an identifier in a program.
///
/// Most of time, a `Label` is used when a reference to a value is needed,
/// however variable names are in grammar to support those cases where
/// a relative label reference is not acceptable, in particular assignments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableName<'a>(pub &'a str);

/// A reference to a location in assembly.
///
/// It can be named or relative. Relative location is a signed integer
/// whose level of depth is determined by a number, negative integers
/// mean backward references, while positive numbers mean forward
/// references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Label<'a> {
    Named(VariableName<'a>),
    Relative(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Opcode<'a> {
    pub name: &'a str,
    pub width: Option<u32>,
    pub mode: OpcodeMode<'a>,
    pub value: Expression<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpcodeMode<'a> {
    Implied, // no argument
    Immediate, // #$
    Address, // $
    Indirect, // ($)
    XIndirect, // ($,x)
    IndirectY, // ($),y
    StackIndirectY, // ($,s),y
    LongIndirect, // [$]
    LongIndirectY, // [$],y
    Move { second: Expression<'a> }, // $,$
    Accumulator, // A
}

/// A single "if" block with predicate and statements.
///
/// This is usually used in a `Vec`, and represents a single predicate along
/// with statements to run if it is met.
#[derive(Debug, Eq, PartialEq)]
pub struct Condition<'a> {
    pub predicate: Option<Expression<'a>>,
    pub statements: Vec<Statement<'a>>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberWidth {
    None,
    OneByte,
    TwoBytes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression<'a> {
    Number(Number),
    Variable(Label<'a>),
    Binary(BinaryOperator, Box<(Expression<'a>, Expression<'a>)>),
    Call(VariableName<'a>, Vec<Expression<'a>>),
}
