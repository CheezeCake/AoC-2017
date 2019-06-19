use std::io;
use std::io::BufRead;
use std::io::BufReader;

struct Instruction {
    opcode: String,
    op1: String,
    op2: Option<String>,
}

struct CPU {
    regs: [i64; 8],
    pc: i64,
    mul_count: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            regs: [0; 8],
            pc: 0,
            mul_count: 0,
        }
    }

    fn get_reg(&self, reg: u8) -> i64 {
        return self.regs[(reg - b'a') as usize];
    }

    fn set_reg(&mut self, reg: &str, val: i64) {
        self.regs[(reg.as_bytes()[0] - b'a') as usize] = val;
    }

    fn imm_or_reg(&self, op: &str) -> i64 {
        let b = op.as_bytes()[0];
        if b.is_ascii_alphabetic() {
            return self.get_reg(b);
        }

        op.parse().unwrap()
    }

    fn sub(&mut self, op1: &str, op2: &str) {
        let result = self.imm_or_reg(op1) - self.imm_or_reg(op2);
        self.set_reg(op1, result);
    }

    fn mul(&mut self, op1: &str, op2: &str) {
        let result = self.imm_or_reg(op1) * self.imm_or_reg(op2);
        self.set_reg(op1, result);
    }

    fn jnz(&mut self, op1: &str, op2: &str) {
        if self.imm_or_reg(op1) != 0 {
            self.pc += self.imm_or_reg(op2) - 1;
        }
    }

    fn exec(&mut self, instr: &Instruction) {
        let op1 = &instr.op1;
        let op2 = instr.op2.as_ref();

        match instr.opcode.as_str() {
            "set" => {
                let op2 = self.imm_or_reg(op2.unwrap());
                self.set_reg(op1, op2);
            }
            "sub" => self.sub(op1, op2.unwrap()),
            "mul" => {
                self.mul(op1, op2.unwrap());
                self.mul_count += 1;
            }
            "jnz" => self.jnz(op1, op2.unwrap()),
            _ => panic!("invalid instruction"),
        };

        self.pc += 1;
    }

    fn running(&self, program: &[Instruction]) -> bool {
        self.pc >= 0 && self.pc < program.len() as i64
    }

    fn run(&mut self, program: &[Instruction]) {
        while self.running(&program) {
            let instr = &program[self.pc as usize];
            self.exec(instr);
        }
    }
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let program: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let lines = line.unwrap();
            let values: Vec<&str> = lines.split_whitespace().collect();

            Instruction {
                opcode: values[0].to_string(),
                op1: values[1].to_string(),
                op2: if values.len() > 2 {
                    Some(values[2].to_string())
                } else {
                    None
                },
            }
        })
        .collect();

    let mut cpu = CPU::new();
    cpu.run(&program);
    println!("part 1: {}", cpu.mul_count);
}
