mod i8080;
mod cpu;
mod memory;

use crate::i8080::emu;

fn main() {
    println!("Intel 8080 emulator");
    
    emu();
}
