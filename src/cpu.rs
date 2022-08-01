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
        }
    }

    pub fn execute(&mut self) {
        println!("Running"); //temp
        self.increment_cycle(1);
    }

    pub fn debug_print_reg(&self) {
        println!(
            "b = {}, c = {}, d = {}, e = {}, h = {}, l = {}, pc = {}, sp= {}",
            self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp
        ); //temp
    }

    fn increment_cycle(&mut self, n: i32) {
        self.cycles += n;
    }
}
