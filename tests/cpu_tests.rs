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

    #[test]
fn test_sta_zero_page() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // Setup: Write the STA Zero Page opcode (0x85) and address 0x10
    cpu.a = 0x55; // Set accumulator value
    memory.write(0x8000, 0x85); // STA Zero Page opcode
    memory.write(0x8001, 0x10); // Zero Page address

    // Initialize the program counter and execute the instruction
    cpu.pc = 0x8000;
    cpu.execute_instruction(&mut memory);

    // Check if the value was stored correctly
    assert_eq!(memory.read(0x0010), 0x55, "Memory at 0x0010 should be 0x55");
}

#[test]
fn test_adc_immediate() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // Setup: Write the ADC Immediate opcode (0x69) and the value to add
    cpu.a = 0x10; // Set initial accumulator value
    memory.write(0x8000, 0x69); // ADC Immediate opcode
    memory.write(0x8001, 0x20); // Value to add (0x20)

    // Initialize the program counter and execute the instruction
    cpu.pc = 0x8000;
    cpu.execute_instruction(&mut memory);

    // Check if the accumulator has the correct value and flags are set properly
    assert_eq!(cpu.a, 0x30, "Accumulator should be 0x30 after ADC");
    assert_eq!(cpu.p & 0b0000_0001, 0, "Carry flag should be cleared");
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();

    // Setup: Write the INX opcode (0xE8)
    cpu.x = 0xFF; // Set initial X register to maximum value
    memory.write(0x8000, 0xE8); // INX opcode

    // Initialize the program counter and execute the instruction
    cpu.pc = 0x8000;
    cpu.execute_instruction(&mut memory);

    // Check if the X register wrapped around and flags are set properly
    assert_eq!(cpu.x, 0x00, "X register should wrap around to 0x00");
    assert_eq!(cpu.p & 0b0000_0010, 0b0000_0010, "Zero flag should be set");
    assert_eq!(cpu.p & 0b1000_0000, 0, "Negative flag should be cleared");
}