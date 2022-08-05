pub fn load_rom(path: String) -> Vec<u8> {
    let rom_file = std::fs::read(path).expect("Error");
    // println!("{:?}", rom_file[0]);
    rom_file
}