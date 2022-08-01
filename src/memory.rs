use std::fs::File;
use std::io::Read;

pub fn load_rom(path: String) {
    let mut rom_file = File::open(path).unwrap();
    let mut buffer = [0u8; 8192];
    rom_file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}