pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u16,
    status: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.a = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                _ => todo!()
            }
        }
    }
}