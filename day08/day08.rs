use std::cmp;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

struct Instruction {
    target: String,
    op: String,
    amount: i32,
    cond_op1: String,
    cond: String,
    cond_op2: String,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(Instruction {
            target: parts[0].to_string(),
            op: parts[1].to_string(),
            amount: parts[2].parse().unwrap(),
            cond_op1: parts[4].to_string(),
            cond: parts[5].to_string(),
            cond_op2: parts[6].to_string(),
        })
    }
}

struct CPU {
    regs: HashMap<String, i32>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            regs: HashMap::new(),
        }
    }

    fn imm_or_reg(&mut self, s: &str) -> i32 {
        let imm = s.parse::<i32>();

        if imm.is_err() {
            let entry = self.regs.entry(s.to_string()).or_insert(0);
            return *entry;
        }

        imm.unwrap()
    }

    fn exec(&mut self, program: &Vec<Instruction>) -> i32 {
        let mut max = 0i32;

        for instr in program {
            let op1 = self.imm_or_reg(&instr.cond_op1);
            let op2 = self.imm_or_reg(&instr.cond_op2);
            let cond = match instr.cond.as_str() {
                "==" => op1 == op2,
                "!=" => op1 != op2,
                ">" => op1 > op2,
                ">=" => op1 >= op2,
                "<" => op1 < op2,
                "<=" => op1 <= op2,
                _ => false,
            };
            if cond {
                let target = self.regs.entry(instr.target.clone()).or_insert(0);
                *target += match instr.op.as_str() {
                    "inc" => instr.amount,
                    _ => -instr.amount,
                };

                max = cmp::max(max, *target);
            }
        }

        max
    }
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let program: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse::<Instruction>().unwrap())
        .collect();

    let mut cpu = CPU::new();
    let max = cpu.exec(&program);

    println!("part 1: {}", cpu.regs.values().max().unwrap());
    println!("part 2: {}", max);
}
