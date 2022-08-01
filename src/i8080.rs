use crate::cpu::CPU;
use crate::memory;

pub fn emu() {
    memory::load_rom("invaders.rom".to_string());

    let mut cpu = CPU::new();

    cpu.debug_print_reg();
    cpu.execute();
}