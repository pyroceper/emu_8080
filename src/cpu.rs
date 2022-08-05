pub struct CPU {
    // registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    // program counter
    pc: u16,
    //stack pointer
    sp: u16,
    cycles: i32,
    pub rom: [u8; 8192],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            cycles: 0,
            rom: [0; 8192],
        }
    }

    pub fn execute(&mut self) {
        println!("Running"); //temp

        let index = self.pc as usize;
        let opcode = format!("{:X}", self.rom[index]);
        self.increment_cycle(1);
        // let opcode = opcode.parse::<i32>().unwrap();
        println!("opcode: {:?}", opcode);

        self.pc += 1;

        match opcode.as_str() {
            "0" => {
                println!("instruction: NOP");
                self.increment_cycle(1);
            }
            "01" => self.lxi_r16_d16(self.b, self.c, 0, 0), // TODO
            _ => println!("instruction: UNKNOWN"),
        }
    }

    pub fn debug_print_reg(&self) {
        println!(
            "b = {}, c = {}, d = {}, e = {}, h = {}, l = {}, pc = {}, sp= {}",
            self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp
        ); //temp
    }

    //helpers
    fn increment_cycle(&mut self, n: i32) {
        self.cycles += n;
    }

    //FIX
    fn fetch_byte(&mut self) -> String {
        self.increment_cycle(1);
        let index = self.pc as usize;
        let byte = format!("{:X}", self.rom[index]);
        self.pc += 1;
        byte
    }

    //instructions
    fn lxi_r16_d16(&mut self, mut r1: u8, mut r2: u8, lo: u8, hi: u8) {
        //TODO
        r1 = lo;
        r2 = hi;
        self.increment_cycle(3);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
