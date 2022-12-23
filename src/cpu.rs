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
        self.pc = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0x00 => {
                    // TODO: implement properly
                    return;
                }
                0xa9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.a = param;

                    if self.a == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }

                    if self.a & 0b1000_0000 == 0b1000_0000 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= 0b0111_1111;
                    }
                }
                0xaa => {
                    self.x = self.a;

                    if self.a == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }

                    if self.a & 0b1000_0000 == 0b1000_0000 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= 0b0111_1111;
                    }
                }
                _ => todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_0xa9_arg_non_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x04, 0x00]);
        assert_eq!(cpu.pc, 3);
        assert_eq!(cpu.a, 0x04);
        assert_eq!(cpu.status, 0b0000_0000)
    }

    #[test]
    fn test_0xa9_arg_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.pc, 3);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.status, 0b0000_0010)
    }

    #[test]
    fn test_0xa9_arg_negative() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0b1011_0010, 0x00]);
        assert_eq!(cpu.pc, 3);
        assert_eq!(cpu.a, 0b1011_0010);
        assert_eq!(cpu.status, 0b1000_0000)
    }

    #[test]
    fn test_0xaa() {
        let mut cpu = CPU::new();
        cpu.a = 0x06;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.x, 0x06);
        assert_eq!(cpu.status, 0b0000_0000);
    }
}