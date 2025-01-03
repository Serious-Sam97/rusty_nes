#[derive(Clone)]
pub struct Opcode {
    pub name: &'static str,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

#[derive(Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Accumulator,
    Relative,
    Implied,
    // Add any additional addressing modes if needed
}

pub fn build_opcode_table() -> [Option<Opcode>; 256] {
    let mut table: [Option<Opcode>; 256] = [(); 256].map(|_| None);

    // ADC - Add with Carry
    table[0x69] = Some(Opcode { name: "ADC", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0x65] = Some(Opcode { name: "ADC", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x75] = Some(Opcode { name: "ADC", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x6D] = Some(Opcode { name: "ADC", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x7D] = Some(Opcode { name: "ADC", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0x79] = Some(Opcode { name: "ADC", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0x61] = Some(Opcode { name: "ADC", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0x71] = Some(Opcode { name: "ADC", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // AND - Logical AND
    table[0x29] = Some(Opcode { name: "AND", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0x25] = Some(Opcode { name: "AND", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x35] = Some(Opcode { name: "AND", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x2D] = Some(Opcode { name: "AND", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x3D] = Some(Opcode { name: "AND", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0x39] = Some(Opcode { name: "AND", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0x21] = Some(Opcode { name: "AND", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0x31] = Some(Opcode { name: "AND", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // ASL - Arithmetic Shift Left
    table[0x0A] = Some(Opcode { name: "ASL", cycles: 2, addressing_mode: AddressingMode::Accumulator });
    table[0x06] = Some(Opcode { name: "ASL", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0x16] = Some(Opcode { name: "ASL", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0x0E] = Some(Opcode { name: "ASL", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0x1E] = Some(Opcode { name: "ASL", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // BCC - Branch if Carry Clear
    table[0x90] = Some(Opcode { name: "BCC", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BCS - Branch if Carry Set
    table[0xB0] = Some(Opcode { name: "BCS", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BEQ - Branch if Equal
    table[0xF0] = Some(Opcode { name: "BEQ", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BIT - Bit Test
    table[0x24] = Some(Opcode { name: "BIT", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x2C] = Some(Opcode { name: "BIT", cycles: 4, addressing_mode: AddressingMode::Absolute });

    // BMI - Branch if Minus
    table[0x30] = Some(Opcode { name: "BMI", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BNE - Branch if Not Equal
    table[0xD0] = Some(Opcode { name: "BNE", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BPL - Branch if Positive
    table[0x10] = Some(Opcode { name: "BPL", cycles: 2, addressing_mode: AddressingMode::Relative });

    // BRK - Force Interrupt
    table[0x00] = Some(Opcode { name: "BRK", cycles: 7, addressing_mode: AddressingMode::Implied });

    // CLC - Clear Carry Flag
    table[0x18] = Some(Opcode { name: "CLC", cycles: 2, addressing_mode: AddressingMode::Implied });

    // CLD - Clear Decimal Mode
    table[0xD8] = Some(Opcode { name: "CLD", cycles: 2, addressing_mode: AddressingMode::Implied });

    // CLI - Clear Interrupt Disable
    table[0x58] = Some(Opcode { name: "CLI", cycles: 2, addressing_mode: AddressingMode::Implied });

    // CLV - Clear Overflow Flag
    table[0xB8] = Some(Opcode { name: "CLV", cycles: 2, addressing_mode: AddressingMode::Implied });

    // CMP - Compare
    table[0xC9] = Some(Opcode { name: "CMP", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xC5] = Some(Opcode { name: "CMP", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xD5] = Some(Opcode { name: "CMP", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0xCD] = Some(Opcode { name: "CMP", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xDD] = Some(Opcode { name: "CMP", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0xD9] = Some(Opcode { name: "CMP", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0xC1] = Some(Opcode { name: "CMP", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0xD1] = Some(Opcode { name: "CMP", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // CPX - Compare X Register
    table[0xE0] = Some(Opcode { name: "CPX", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xE4] = Some(Opcode { name: "CPX", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xEC] = Some(Opcode { name: "CPX", cycles: 4, addressing_mode: AddressingMode::Absolute });

    // CPY - Compare Y Register
    table[0xC0] = Some(Opcode { name: "CPY", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xC4] = Some(Opcode { name: "CPY", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xCC] = Some(Opcode { name: "CPY", cycles: 4, addressing_mode: AddressingMode::Absolute });

    // DEC - Decrement Memory
    table[0xC6] = Some(Opcode { name: "DEC", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0xD6] = Some(Opcode { name: "DEC", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0xCE] = Some(Opcode { name: "DEC", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0xDE] = Some(Opcode { name: "DEC", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // DEX - Decrement X Register
    table[0xCA] = Some(Opcode { name: "DEX", cycles: 2, addressing_mode: AddressingMode::Implied });

    // DEY - Decrement Y Register
    table[0x88] = Some(Opcode { name: "DEY", cycles: 2, addressing_mode: AddressingMode::Implied });

    // EOR - Exclusive OR
    table[0x49] = Some(Opcode { name: "EOR", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0x45] = Some(Opcode { name: "EOR", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x55] = Some(Opcode { name: "EOR", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x4D] = Some(Opcode { name: "EOR", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x5D] = Some(Opcode { name: "EOR", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0x59] = Some(Opcode { name: "EOR", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0x41] = Some(Opcode { name: "EOR", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0x51] = Some(Opcode { name: "EOR", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // INC - Increment Memory
    table[0xE6] = Some(Opcode { name: "INC", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0xF6] = Some(Opcode { name: "INC", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0xEE] = Some(Opcode { name: "INC", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0xFE] = Some(Opcode { name: "INC", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // INX - Increment X Register
    table[0xE8] = Some(Opcode { name: "INX", cycles: 2, addressing_mode: AddressingMode::Implied });

    // INY - Increment Y Register
    table[0xC8] = Some(Opcode { name: "INY", cycles: 2, addressing_mode: AddressingMode::Implied });

    // JMP - Jump
    table[0x4C] = Some(Opcode { name: "JMP", cycles: 3, addressing_mode: AddressingMode::Absolute });
    table[0x6C] = Some(Opcode { name: "JMP", cycles: 5, addressing_mode: AddressingMode::Indirect });

    // JSR - Jump to Subroutine
    table[0x20] = Some(Opcode { name: "JSR", cycles: 6, addressing_mode: AddressingMode::Absolute });

    // LDA - Load Accumulator
    table[0xA9] = Some(Opcode { name: "LDA", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xA5] = Some(Opcode { name: "LDA", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xB5] = Some(Opcode { name: "LDA", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0xAD] = Some(Opcode { name: "LDA", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xBD] = Some(Opcode { name: "LDA", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0xB9] = Some(Opcode { name: "LDA", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0xA1] = Some(Opcode { name: "LDA", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0xB1] = Some(Opcode { name: "LDA", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // LDX - Load X Register
    table[0xA2] = Some(Opcode { name: "LDX", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xA6] = Some(Opcode { name: "LDX", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xB6] = Some(Opcode { name: "LDX", cycles: 4, addressing_mode: AddressingMode::ZeroPageY });
    table[0xAE] = Some(Opcode { name: "LDX", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xBE] = Some(Opcode { name: "LDX", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });

    // LDY - Load Y Register
    table[0xA0] = Some(Opcode { name: "LDY", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xA4] = Some(Opcode { name: "LDY", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xB4] = Some(Opcode { name: "LDY", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0xAC] = Some(Opcode { name: "LDY", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xBC] = Some(Opcode { name: "LDY", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });

    // LSR - Logical Shift Right
    table[0x4A] = Some(Opcode { name: "LSR", cycles: 2, addressing_mode: AddressingMode::Accumulator });
    table[0x46] = Some(Opcode { name: "LSR", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0x56] = Some(Opcode { name: "LSR", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0x4E] = Some(Opcode { name: "LSR", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0x5E] = Some(Opcode { name: "LSR", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // NOP - No Operation
    table[0xEA] = Some(Opcode { name: "NOP", cycles: 2, addressing_mode: AddressingMode::Implied });

    // ORA - Logical Inclusive OR
    table[0x09] = Some(Opcode { name: "ORA", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0x05] = Some(Opcode { name: "ORA", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x15] = Some(Opcode { name: "ORA", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x0D] = Some(Opcode { name: "ORA", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x1D] = Some(Opcode { name: "ORA", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0x19] = Some(Opcode { name: "ORA", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0x01] = Some(Opcode { name: "ORA", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0x11] = Some(Opcode { name: "ORA", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // PHA - Push Accumulator
    table[0x48] = Some(Opcode { name: "PHA", cycles: 3, addressing_mode: AddressingMode::Implied });

    // PHP - Push Processor Status
    table[0x08] = Some(Opcode { name: "PHP", cycles: 3, addressing_mode: AddressingMode::Implied });

    // PLA - Pull Accumulator
    table[0x68] = Some(Opcode { name: "PLA", cycles: 4, addressing_mode: AddressingMode::Implied });

    // PLP - Pull Processor Status
    table[0x28] = Some(Opcode { name: "PLP", cycles: 4, addressing_mode: AddressingMode::Implied });

    // ROL - Rotate Left
    table[0x2A] = Some(Opcode { name: "ROL", cycles: 2, addressing_mode: AddressingMode::Accumulator });
    table[0x26] = Some(Opcode { name: "ROL", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0x36] = Some(Opcode { name: "ROL", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0x2E] = Some(Opcode { name: "ROL", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0x3E] = Some(Opcode { name: "ROL", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // ROR - Rotate Right
    table[0x6A] = Some(Opcode { name: "ROR", cycles: 2, addressing_mode: AddressingMode::Accumulator });
    table[0x66] = Some(Opcode { name: "ROR", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0x76] = Some(Opcode { name: "ROR", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0x6E] = Some(Opcode { name: "ROR", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0x7E] = Some(Opcode { name: "ROR", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });

    // RTI - Return from Interrupt
    table[0x40] = Some(Opcode { name: "RTI", cycles: 6, addressing_mode: AddressingMode::Implied });

    // RTS - Return from Subroutine
    table[0x60] = Some(Opcode { name: "RTS", cycles: 6, addressing_mode: AddressingMode::Implied });

    // SBC - Subtract with Carry
    table[0xE9] = Some(Opcode { name: "SBC", cycles: 2, addressing_mode: AddressingMode::Immediate });
    table[0xE5] = Some(Opcode { name: "SBC", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xF5] = Some(Opcode { name: "SBC", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0xED] = Some(Opcode { name: "SBC", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xFD] = Some(Opcode { name: "SBC", cycles: 4, addressing_mode: AddressingMode::AbsoluteX });
    table[0xF9] = Some(Opcode { name: "SBC", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0xE1] = Some(Opcode { name: "SBC", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0xF1] = Some(Opcode { name: "SBC", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // SEC - Set Carry Flag
    table[0x38] = Some(Opcode { name: "SEC", cycles: 2, addressing_mode: AddressingMode::Implied });

    // SED - Set Decimal Flag
    table[0xF8] = Some(Opcode { name: "SED", cycles: 2, addressing_mode: AddressingMode::Implied });

    // SEI - Set Interrupt Disable
    table[0x78] = Some(Opcode { name: "SEI", cycles: 2, addressing_mode: AddressingMode::Implied });

    // STA - Store Accumulator
    table[0x85] = Some(Opcode { name: "STA", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x95] = Some(Opcode { name: "STA", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x8D] = Some(Opcode { name: "STA", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x9D] = Some(Opcode { name: "STA", cycles: 5, addressing_mode: AddressingMode::AbsoluteX });
    table[0x99] = Some(Opcode { name: "STA", cycles: 5, addressing_mode: AddressingMode::AbsoluteY });
    table[0x81] = Some(Opcode { name: "STA", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0x91] = Some(Opcode { name: "STA", cycles: 6, addressing_mode: AddressingMode::IndirectIndexed });

    // STX - Store X Register
    table[0x86] = Some(Opcode { name: "STX", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x96] = Some(Opcode { name: "STX", cycles: 4, addressing_mode: AddressingMode::ZeroPageY });
    table[0x8E] = Some(Opcode { name: "STX", cycles: 4, addressing_mode: AddressingMode::Absolute });

    // STY - Store Y Register
    table[0x84] = Some(Opcode { name: "STY", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x94] = Some(Opcode { name: "STY", cycles: 4, addressing_mode: AddressingMode::ZeroPageX });
    table[0x8C] = Some(Opcode { name: "STY", cycles: 4, addressing_mode: AddressingMode::Absolute });

    // TAX - Transfer Accumulator to X
    table[0xAA] = Some(Opcode { name: "TAX", cycles: 2, addressing_mode: AddressingMode::Implied });

    // TAY - Transfer Accumulator to Y
    table[0xA8] = Some(Opcode { name: "TAY", cycles: 2, addressing_mode: AddressingMode::Implied });

    // TSX - Transfer Stack Pointer to X
    table[0xBA] = Some(Opcode { name: "TSX", cycles: 2, addressing_mode: AddressingMode::Implied });

    // TXA - Transfer X to Accumulator
    table[0x8A] = Some(Opcode { name: "TXA", cycles: 2, addressing_mode: AddressingMode::Implied });

    // TXS - Transfer X to Stack Pointer
    table[0x9A] = Some(Opcode { name: "TXS", cycles: 2, addressing_mode: AddressingMode::Implied });

    // TYA - Transfer Y to Accumulator
    table[0x98] = Some(Opcode { name: "TYA", cycles: 2, addressing_mode: AddressingMode::Implied });

    // STP - Stop (Unofficial)
    table[0xDB] = Some(Opcode { name: "STP", cycles: 0, addressing_mode: AddressingMode::Implied });

    // SHX - Store X Indexed with Shift (Unofficial)
    table[0x9E] = Some(Opcode { name: "SHX", cycles: 0, addressing_mode: AddressingMode::AbsoluteY });
    
    //OFICIAL OPCODES COMPLETED


    //UNNOFFICIAL OPCODES
    // LAX - Load Accumulator and X
    table[0xA7] = Some(Opcode { name: "LAX", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0xB7] = Some(Opcode { name: "LAX", cycles: 4, addressing_mode: AddressingMode::ZeroPageY });
    table[0xAF] = Some(Opcode { name: "LAX", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0xBF] = Some(Opcode { name: "LAX", cycles: 4, addressing_mode: AddressingMode::AbsoluteY });
    table[0xA3] = Some(Opcode { name: "LAX", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });
    table[0xB3] = Some(Opcode { name: "LAX", cycles: 5, addressing_mode: AddressingMode::IndirectIndexed });

    // SAX - Store A & X
    table[0x87] = Some(Opcode { name: "SAX", cycles: 3, addressing_mode: AddressingMode::ZeroPage });
    table[0x97] = Some(Opcode { name: "SAX", cycles: 4, addressing_mode: AddressingMode::ZeroPageY });
    table[0x8F] = Some(Opcode { name: "SAX", cycles: 4, addressing_mode: AddressingMode::Absolute });
    table[0x83] = Some(Opcode { name: "SAX", cycles: 6, addressing_mode: AddressingMode::IndexedIndirect });

    // DCP - Decrement Memory and Compare
    table[0xC7] = Some(Opcode { name: "DCP", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0xD7] = Some(Opcode { name: "DCP", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0xCF] = Some(Opcode { name: "DCP", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0xDF] = Some(Opcode { name: "DCP", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });
    table[0xDB] = Some(Opcode { name: "DCP", cycles: 7, addressing_mode: AddressingMode::AbsoluteY });
    table[0xD3] = Some(Opcode { name: "DCP", cycles: 8, addressing_mode: AddressingMode::IndirectIndexed });
    table[0xC3] = Some(Opcode { name: "DCP", cycles: 8, addressing_mode: AddressingMode::IndexedIndirect });

    // ISC - Increment Memory and Subtract with Carry
    table[0xE7] = Some(Opcode { name: "ISC", cycles: 5, addressing_mode: AddressingMode::ZeroPage });
    table[0xF7] = Some(Opcode { name: "ISC", cycles: 6, addressing_mode: AddressingMode::ZeroPageX });
    table[0xEF] = Some(Opcode { name: "ISC", cycles: 6, addressing_mode: AddressingMode::Absolute });
    table[0xFF] = Some(Opcode { name: "ISC", cycles: 7, addressing_mode: AddressingMode::AbsoluteX });
    table[0xFB] = Some(Opcode { name: "ISC", cycles: 7, addressing_mode: AddressingMode::AbsoluteY });
    table[0xF3] = Some(Opcode { name: "ISC", cycles: 8, addressing_mode: AddressingMode::IndirectIndexed });
    table[0xE3] = Some(Opcode { name: "ISC", cycles: 8, addressing_mode: AddressingMode::IndexedIndirect });

    table
}