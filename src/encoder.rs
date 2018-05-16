#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Implied,
    DirectPage,              // dp
    Absolute,                // addr
    AbsoluteLong,            // long
    Immediate,               // #const
    DpIndexedX,              // dp,x
    AbsoluteIndexedX,        // addr,x
    AbsoluteIndexedY,        // addr,y
    AbsoluteLongIndexedX,    // long,x
    DpIndirect,              // (dp)
    DpIndexedIndirectX,      // (dp,x)
    DpIndirectIndexedIndexY, // (dp),y
    DpIndirectLong,          // [dp]
    DpIndirectLongIndexedY,  // [dp],y
    StackRelative,           // sr,s
    SrIndirectIndexedY,      // (sr,s),y
}

use self::AddressingMode::*;

pub fn get_opcode(name: &str, addressing_mode: AddressingMode) -> Option<u8> {
    Some(match (name, addressing_mode) {
        ("ADC", DirectPage) => 0x65,
        ("ADC", Absolute) => 0x6D,
        ("ADC", AbsoluteLong) => 0x6F,
        ("ADC", Immediate) => 0x69,
        ("ADC", DpIndexedX) => 0x75,
        ("ADC", AbsoluteIndexedX) => 0x7D,
        ("ADC", AbsoluteIndexedY) => 0x79,
        ("ADC", AbsoluteLongIndexedX) => 0x7F,
        ("ADC", DpIndirect) => 0x72,
        ("ADC", DpIndexedIndirectX) => 0x61,
        ("ADC", DpIndirectIndexedIndexY) => 0x71,
        ("ADC", DpIndirectLong) => 0x67,
        ("ADC", DpIndirectLongIndexedY) => 0x77,
        ("ADC", StackRelative) => 0x63,
        ("ADC", SrIndirectIndexedY) => 0x7E,
        _ => return None,
    })
}
