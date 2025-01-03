use crate::memory::Memory;
use crate::opcodes::{build_opcode_table, AddressingMode};

pub struct CPU {
    pub a: u8, //Accumulator
    pub y: u8, //Index Register Y
    pub x: u8, //Index Register X
    pub pc: u16, //Program Counter
    pub sp: u8, //Stack Pointer
    pub p:  u8, //Status Register
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

    fn get_operand_address(&self, memory: &Memory, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => memory.read(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let addr = memory.read(self.pc);
                addr.wrapping_add(self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = memory.read(self.pc);
                addr.wrapping_add(self.y) as u16
            }
            AddressingMode::Absolute => {
                let low_byte = memory.read(self.pc) as u16;
                let high_byte = memory.read(self.pc + 1) as u16;
                (high_byte << 8) | low_byte
            }
            AddressingMode::AbsoluteX => {
                let low_byte = memory.read(self.pc) as u16;
                let high_byte = memory.read(self.pc + 1) as u16;
                ((high_byte << 8) | low_byte).wrapping_add(self.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let low_byte = memory.read(self.pc) as u16;
                let high_byte = memory.read(self.pc + 1) as u16;
                ((high_byte << 8) | low_byte).wrapping_add(self.y as u16)
            }
            AddressingMode::Indirect => {
                let ptr = memory.read(self.pc) as u16 | (memory.read(self.pc + 1) as u16) << 8;
                let low_byte = memory.read(ptr) as u16;
                let high_byte = memory.read((ptr & 0xFF00) | ((ptr + 1) & 0x00FF)) as u16;
                (high_byte << 8) | low_byte
            }
            AddressingMode::IndexedIndirect => {
                let addr = memory.read(self.pc).wrapping_add(self.x) as u16;
                let low_byte = memory.read(addr) as u16;
                let high_byte = memory.read((addr + 1) & 0x00FF) as u16;
                (high_byte << 8) | low_byte
            }
            AddressingMode::IndirectIndexed => {
                let base = memory.read(self.pc) as u16;
                let low_byte = memory.read(base) as u16;
                let high_byte = memory.read((base + 1) & 0x00FF) as u16;
                ((high_byte << 8) | low_byte).wrapping_add(self.y as u16)
            }
            AddressingMode::Accumulator => 0,
            AddressingMode::Relative => {
                let offset = memory.read(self.pc) as i8;
                self.pc.wrapping_add(1).wrapping_add(offset as u16)
            }
            AddressingMode::Implied => 0,
        }
    }

    pub fn execute_instruction(&mut self, memory: &mut Memory) {
         let opcode = memory.read(self.pc); // Fetch the opcode
        self.pc += 1; // Increment PC to the next byte

        let opcode_table = build_opcode_table(); // Fetch opcode table
        if let Some(opcode_data) = opcode_table[opcode as usize] {
            match opcode_data.name {
                "LDA" => self.lda(memory, &opcode_data.addressing_mode),
                "STA" => self.sta(memory, &opcode_data.addressing_mode),
                "ADC" => self.adc(memory, &opcode_data.addressing_mode),
                "AND" => self.and(memory, &opcode_data.addressing_mode),
                "ORA" => self.ora(memory, &opcode_data.addressing_mode),
                "EOR" => self.eor(memory, &opcode_data.addressing_mode),
                "CMP" => self.cmp(memory, &opcode_data.addressing_mode),
                "CPX" => self.cpx(memory, &opcode_data.addressing_mode),
                "CPY" => self.cpy(memory, &opcode_data.addressing_mode),
                "INC" => self.inc(memory, &opcode_data.addressing_mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "DEC" => self.dec(memory, &opcode_data.addressing_mode),
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "PHA" => self.pha(memory),
                "PHP" => self.php(memory),
                "PLA" => self.pla(memory),
                "PLP" => self.plp(memory),
                "BEQ" => self.beq(memory),
                "BNE" => self.bne(memory),
                "BMI" => self.bmi(memory),
                "BPL" => self.bpl(memory),
                "BCS" => self.bcs(memory),
                "BCC" => self.bcc(memory),
                "BVS" => self.bvs(memory),
                "BVC" => self.bvc(memory),
                "JMP" => self.jmp(memory, &opcode_data.addressing_mode),
                "JSR" => self.jsr(memory),
                "RTS" => self.rts(memory),
                "CLC" => self.clc(),
                "SEC" => self.sec(),
                "CLI" => self.cli(),
                "SEI" => self.sei(),
                "CLV" => self.clv(),
                "CLD" => self.cld(),
                "SED" => self.sed(),
                "NOP" => self.nop(),
                "BRK" => self.brk(memory),
                "RTI" => self.rti(memory),
                // Add cases for other opcodes
                _ => panic!("Unimplemented opcode: {}", opcode_data.name),
            }
        } else {
            panic!("Unknown opcode: 0x{:X}", opcode);
        }
    }

    fn lda(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        self.a = memory.read(addr);
        self.set_zero_and_negative_flags(self.a);
    }

    fn sta(&mut self, memory: &mut Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        memory.write(addr, self.a);
    }

    fn adc(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let value = memory.read(addr);
        let carry = self.p & 0x01;
        let result = self.a as u16 + value as u16 + carry as u16;

        self.a = result as u8;
        self.set_zero_and_negative_flags(self.a);

        if result > 0xFF {
            self.p |= 0x01; // Set carry flag
        } else {
            self.p &= !0x01; // Clear carry flag
        }
    }

    // Helper method for setting zero and negative flags
    fn set_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.p |= 0x02; // Set zero flag
        } else {
            self.p &= !0x02; // Clear zero flag
        }
        if result & 0x80 != 0 {
            self.p |= 0x80; // Set negative flag
        } else {
            self.p &= !0x80; // Clear negative flag
        }
    }

    fn and(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        self.a &= memory.read(addr);
        self.set_zero_and_negative_flags(self.a);
    }
    
    fn ora(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        self.a |= memory.read(addr);
        self.set_zero_and_negative_flags(self.a);
    }
    
    fn eor(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        self.a ^= memory.read(addr);
        self.set_zero_and_negative_flags(self.a);
    }

    fn cmp(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let value = memory.read(addr);
        let result = self.a.wrapping_sub(value);
    
        self.set_zero_and_negative_flags(result);
        if self.a >= value {
            self.p |= 0x01; // Set carry flag
        } else {
            self.p &= !0x01; // Clear carry flag
        }
    }
    
    fn cpx(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let value = memory.read(addr);
        let result = self.x.wrapping_sub(value);
    
        self.set_zero_and_negative_flags(result);
        if self.x >= value {
            self.p |= 0x01; // Set carry flag
        } else {
            self.p &= !0x01; // Clear carry flag
        }
    }
    
    fn cpy(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let value = memory.read(addr);
        let result = self.y.wrapping_sub(value);
    
        self.set_zero_and_negative_flags(result);
        if self.y >= value {
            self.p |= 0x01; // Set carry flag
        } else {
            self.p &= !0x01; // Clear carry flag
        }
    }

    fn inc(&mut self, memory: &mut Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let mut value = memory.read(addr);
        value = value.wrapping_add(1);
        memory.write(addr, value);
        self.set_zero_and_negative_flags(value);
    }
    
    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_zero_and_negative_flags(self.x);
    }
    
    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_zero_and_negative_flags(self.y);
    }
    
    fn dec(&mut self, memory: &mut Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        let mut value = memory.read(addr);
        value = value.wrapping_sub(1);
        memory.write(addr, value);
        self.set_zero_and_negative_flags(value);
    }
    
    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_zero_and_negative_flags(self.x);
    }
    
    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_zero_and_negative_flags(self.y);
    }

    fn pha(&mut self, memory: &mut Memory) {
        memory.write(0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);
    }
    
    fn php(&mut self, memory: &mut Memory) {
        memory.write(0x0100 + self.sp as u16, self.p);
        self.sp = self.sp.wrapping_sub(1);
    }
    
    fn pla(&mut self, memory: &Memory) {
        self.sp = self.sp.wrapping_add(1);
        self.a = memory.read(0x0100 + self.sp as u16);
        self.set_zero_and_negative_flags(self.a);
    }
    
    fn plp(&mut self, memory: &Memory) {
        self.sp = self.sp.wrapping_add(1);
        self.p = memory.read(0x0100 + self.sp as u16);
    }

    fn beq(&mut self, memory: &Memory) {
        if self.p & 0x02 != 0 { // Check if zero flag is set
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bne(&mut self, memory: &Memory) {
        if self.p & 0x02 == 0 { // Check if zero flag is clear
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bmi(&mut self, memory: &Memory) {
        if self.p & 0x80 != 0 { // Check if negative flag is set
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bpl(&mut self, memory: &Memory) {
        if self.p & 0x80 == 0 { // Check if negative flag is clear
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bcs(&mut self, memory: &Memory) {
        if self.p & 0x01 != 0 { // Check if carry flag is set
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bcc(&mut self, memory: &Memory) {
        if self.p & 0x01 == 0 { // Check if carry flag is clear
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bvs(&mut self, memory: &Memory) {
        if self.p & 0x40 != 0 { // Check if overflow flag is set
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }
    
    fn bvc(&mut self, memory: &Memory) {
        if self.p & 0x40 == 0 { // Check if overflow flag is clear
            let addr = self.get_operand_address(memory, &AddressingMode::Relative);
            self.pc = addr;
        }
    }

    fn jmp(&mut self, memory: &Memory, mode: &AddressingMode) {
        let addr = self.get_operand_address(memory, mode);
        self.pc = addr;
    }
    
    fn jsr(&mut self, memory: &mut Memory) {
        let addr = self.get_operand_address(memory, &AddressingMode::Absolute);
        self.sp = self.sp.wrapping_sub(1);
        memory.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0xFF) as u8); // Push high byte of PC
        self.sp = self.sp.wrapping_sub(1);
        memory.write(0x0100 + self.sp as u16, (self.pc & 0xFF) as u8); // Push low byte of PC
        self.pc = addr;
    }
    
    fn rts(&mut self, memory: &Memory) {
        let low_byte = memory.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high_byte = memory.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc = (high_byte << 8) | low_byte;
    }

    fn clc(&mut self) {
        self.p &= !0x01; // Clear carry flag
    }
    
    fn sec(&mut self) {
        self.p |= 0x01; // Set carry flag
    }
    
    fn cli(&mut self) {
        self.p &= !0x04; // Clear interrupt disable flag
    }
    
    fn sei(&mut self) {
        self.p |= 0x04; // Set interrupt disable flag
    }
    
    fn clv(&mut self) {
        self.p &= !0x40; // Clear overflow flag
    }
    
    fn cld(&mut self) {
        self.p &= !0x08; // Clear decimal mode flag (not used in NES)
    }
    
    fn sed(&mut self) {
        self.p |= 0x08; // Set decimal mode flag (not used in NES)
    }

    fn nop(&mut self) {
        // No operation, just increment PC or do nothing
    }
    
    fn brk(&mut self, memory: &mut Memory) {
        self.pc += 1;
        memory.write(0x0100 + self.sp as u16, (self.pc >> 8) as u8); // Push high byte of PC
        self.sp = self.sp.wrapping_sub(1);
        memory.write(0x0100 + self.sp as u16, (self.pc & 0xFF) as u8); // Push low byte of PC
        self.sp = self.sp.wrapping_sub(1);
        memory.write(0x0100 + self.sp as u16, self.p | 0x10); // Push status register with B flag set
        self.sp = self.sp.wrapping_sub(1);
        self.p |= 0x04; // Set interrupt disable flag
        self.pc = (memory.read(0xFFFF) as u16) << 8 | memory.read(0xFFFE) as u16; // Fetch IRQ vector
    }
    
    fn rti(&mut self, memory: &Memory) {
        self.sp = self.sp.wrapping_add(1);
        self.p = memory.read(0x0100 + self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        let low_byte = memory.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high_byte = memory.read(0x0100 + self.sp as u16) as u16;
        self.pc = (high_byte << 8) | low_byte;
    }

}