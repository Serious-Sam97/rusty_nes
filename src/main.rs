pub mod cpu;
pub mod memory;
pub mod opcodes;

use cpu::CPU;
use memory::Memory;

fn main() {
    let memory = Memory::new();
    let mut cpu = CPU::new();

    cpu.reset(&memory);
}
