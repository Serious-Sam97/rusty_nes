use rusty_nes::cpu::CPU;
use rusty_nes::memory::Memory;

#[test]
fn test_cpu_reset() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    memory.write(0xFFFC, 0x00);
    memory.write(0xFFFD, 0x80);

    cpu.reset(&memory);

    assert_eq!(cpu.get_pc(), 0x8000, "PC should be set to 0x8000 after reset");
}

#[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Setup: Write the LDA Immediate opcode (0xA9) and the value to load into memory
        memory.write(0x8000, 0xA9); // LDA Immediate opcode
        memory.write(0x8001, 0x42); // Value to load (0x42)

        // Initialize the program counter and execute the instruction
        cpu.pc = 0x8000;
        cpu.execute_instruction(&mut memory);

        // Check if the accumulator has the correct value and flags are set properly
        assert_eq!(cpu.a, 0x42, "Accumulator should be 0x42 after LDA");
        assert_eq!(cpu.p & 0b0000_0010, 0, "Zero flag should be cleared");
        assert_eq!(cpu.p & 0b1000_0000, 0, "Negative flag should be cleared");
    }