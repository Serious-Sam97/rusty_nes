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