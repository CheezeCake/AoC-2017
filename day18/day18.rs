use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

struct Instruction {
    opcode: String,
    op1: String,
    op2: Option<String>,
}

struct CPU {
    regs: [i64; 26],
    pc: i64,
    last_freq_play: i64,
    wait_queue: VecDeque<i64>,
    send_count: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            regs: [0; 26],
            pc: 0,
            last_freq_play: 0,
            wait_queue: VecDeque::new(),
            send_count: 0,
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

    fn add(&mut self, op1: &str, op2: &str) {
        let result = self.imm_or_reg(op1) + self.imm_or_reg(op2);
        self.set_reg(op1, result);
    }

    fn mul(&mut self, op1: &str, op2: &str) {
        let result = self.imm_or_reg(op1) * self.imm_or_reg(op2);
        self.set_reg(op1, result);
    }

    fn modulo(&mut self, op1: &str, op2: &str) {
        let result = self.imm_or_reg(op1) % self.imm_or_reg(op2);
        self.set_reg(op1, result);
    }

    fn jgz(&mut self, op1: &str, op2: &str) {
        if self.imm_or_reg(op1) > 0 {
            self.pc += self.imm_or_reg(op2) - 1;
        }
    }

    fn exec(&mut self, instr: &Instruction) -> bool {
        let op1 = &instr.op1;
        let op2 = instr.op2.clone();

        match instr.opcode.as_str() {
            "set" => {
                let op2 = self.imm_or_reg(&op2.unwrap());
                self.set_reg(op1, op2);
            }
            "add" => self.add(op1, &op2.unwrap()),
            "mul" => self.mul(op1, &op2.unwrap()),
            "mod" => self.modulo(op1, &op2.unwrap()),
            "jgz" => self.jgz(op1, &op2.unwrap()),
            _ => return false,
        };

        self.pc += 1;
        true
    }

    fn exec_v1(&mut self, instr: &Instruction) -> bool {
        if self.exec(instr) {
            return true;
        }

        match instr.opcode.as_str() {
            "snd" => self.last_freq_play = self.imm_or_reg(&instr.op1),
            "rcv" => {
                if self.imm_or_reg(&instr.op1) != 0 {
                    return false;
                }
            }
            _ => (),
        };

        self.pc += 1;
        true
    }

    fn exec_v2(&mut self, instr: &Instruction, other: &mut CPU) -> bool {
        if self.exec(instr) {
            return true;
        }

        match instr.opcode.as_str() {
            "snd" => {
                other.wait_queue.push_back(self.imm_or_reg(&instr.op1));
                self.send_count += 1;
            }
            "rcv" => {
                if let Some(rcv) = self.wait_queue.pop_front() {
                    self.set_reg(&instr.op1, rcv);
                } else {
                    return false;
                }
            }
            _ => (),
        };

        self.pc += 1;
        true
    }

    fn running(&self, program: &[Instruction]) -> bool {
        self.pc >= 0 && self.pc < program.len() as i64
    }
}

fn recovered_frequency(program: &[Instruction]) -> i64 {
    let mut cpu = CPU::new();

    while cpu.running(&program) {
        let instr = &program[cpu.pc as usize];
        if !cpu.exec_v1(instr) {
            break;
        }
    }

    cpu.last_freq_play
}

fn send_count(program: &[Instruction]) -> usize {
    let mut cpu0 = CPU::new();
    cpu0.set_reg("p", 0);
    let mut cpu1 = CPU::new();
    cpu1.set_reg("p", 1);

    while cpu0.running(&program) && cpu1.running(&program) {
        let instr0 = &program[cpu0.pc as usize];
        let instr1 = &program[cpu1.pc as usize];
        let ok0 = cpu0.exec_v2(instr0, &mut cpu1);
        let ok1 = cpu1.exec_v2(instr1, &mut cpu0);
        if !ok0 && !ok1 {
            break;
        }
    }

    cpu1.send_count
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

    println!("part 1: {}", recovered_frequency(&program));
    println!("part 2: {}", send_count(&program));
}
