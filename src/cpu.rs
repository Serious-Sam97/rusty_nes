use crate::memory::Memory;
pub struct CPU {
    a: u8, //Accumulator
    y: u8, //Index Register Y
    x: u8, //Index Register X
    pc: u16, //Program Counter
    sp: u8, //Stack Pointer
    p:  u8, //Status Register
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000, // This should be set by a reset method fetching the vector from memory.
            sp: 0xFD,   // Common startup value for the stack pointer.
            p: 0x34,    // Initial status register value (interrupts disabled).
        }
    }

    pub fn reset(&mut self, memory: &Memory) -> () {
        let low_byte = memory.read(0xFFFC);
        let high_byte = memory.read(0xFFFD);

        self.pc = (high_byte as u16) << 8 | (low_byte as u16);
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }
}