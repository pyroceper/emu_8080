use crate::cpu::CPU;
use crate::memory;

pub fn emu() {

    let mut cpu = CPU::new();

    cpu.rom = memory::load_rom("invaders.rom".to_string());

    let mut input_buffer = String::new();
    loop {
        cpu.debug_print_reg();
        cpu.execute();
        std::io::stdin().read_line(&mut input_buffer)
            .expect("Failed to read input!");
    }
}