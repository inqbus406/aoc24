use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut machine = Machine::from_file("input/day17.txt")?;
    let orig_reg_b = machine.reg_b;
    let orig_reg_c = machine.reg_c;
    let program = machine.program.clone();
    let mut outputs = machine.run();
    print!("Part1: ");
    print!("{}", outputs[0]);
    for output in &outputs[1..] {
        print!(",{}", output);
    }
    println!();

    // Part 2:

    // Seems like each output n changes with every 2^(n+1)
    // 7583068569 was too low
    // 216133732885152 too low...
    let mut reg_a: usize = 0;
    // for (n, output) in program.iter().enumerate().rev() {
    //     machine.reg_a = reg_a;
    //     machine.reg_b = orig_reg_b;
    //     machine.reg_c = orig_reg_c;
    //     outputs = machine.run();
    //     while outputs.len() - 1 < n || outputs[n] != *output as usize {
    //     // while outputs[n] != *output as usize {
    //         reg_a += 8_usize.pow(n as u32) % 8_usize.pow((n + 1) as u32);
    //         machine.reg_a = reg_a;
    //         machine.reg_b = orig_reg_b;
    //         machine.reg_c = orig_reg_c;
    //         outputs = machine.run();
    //     }
    //     dbg!(&outputs);
    //     dbg!(&reg_a);
    // }

    let mut factors = vec![0; program.len()];
    loop {
        reg_a = 0;
        for (i, factor) in factors.iter().enumerate() {
            reg_a += 8usize.pow(i as u32) * factor;
        }
        machine.reg_a = reg_a;
        machine.reg_b = orig_reg_b;
        machine.reg_c = orig_reg_c;
        let outputs = machine.run();

        if outputs
            .iter()
            .zip(program.iter())
            .all(|(&num1, &num2)| num1 == num2 as usize)
        {
            break;
        }

        for i in (0..program.len()).rev() {
            if outputs.len() < i || outputs[i] != program[i] as usize {
                factors[i] += 1;
                break;
            }
        }
    }

    // loop {
    //     reg_a += 1;
    //     machine.reg_a = reg_a;
    //     machine.reg_b = orig_reg_b;
    //     machine.reg_c = orig_reg_c;
    //     outputs = machine.run();
    //     if outputs.len() == machine.program.len()
    //         && outputs.iter().zip(&machine.program).all(|(&num1, &num2)| num1 == num2 as usize) {
    //         break;
    //     }
    //     println!("reg_a: {}", reg_a);
    //     dbg!(&outputs);
    // }
    // dbg!(&outputs);

    println!("Part2: {}", reg_a);

    Ok(())
}

#[derive(Debug)]
struct Machine {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<u8>,
}

impl Machine {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;
        let mut program = Vec::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "Register" => match tokens[1] {
                    "A:" => reg_a = tokens.last().unwrap().parse::<usize>().unwrap(),
                    "B:" => reg_b = tokens.last().unwrap().parse::<usize>().unwrap(),
                    "C:" => reg_c = tokens.last().unwrap().parse::<usize>().unwrap(),
                    _ => unreachable!(),
                },
                "Program:" => {
                    program = tokens[1]
                        .split(',')
                        .map(|s| s.parse::<u8>().unwrap())
                        .collect();
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            reg_a,
            reg_b,
            reg_c,
            program,
        })
    }

    fn run(&mut self) -> Vec<usize> {
        let mut outputs = Vec::new();
        let mut instruction_pointer = 0;

        while instruction_pointer < self.program.len() - 1 {
            // dbg!(instruction_pointer);
            // dbg!(&self.reg_a);
            let instruction = Instruction::from_val(self.program[instruction_pointer]).unwrap();
            let literal_operand = self.program[instruction_pointer + 1];
            let combo_operand = self.combo_operand(literal_operand);

            match instruction {
                Instruction::Adv => {
                    self.reg_a = self.reg_a >> combo_operand;
                }
                Instruction::Bxl => {
                    self.reg_b = self.reg_b ^ literal_operand as usize;
                }
                Instruction::Bst => {
                    self.reg_b = (combo_operand % 8) as usize;
                }
                Instruction::Jnz => {
                    if self.reg_a != 0 {
                        instruction_pointer = literal_operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                }
                Instruction::Out => {
                    let result = combo_operand % 8;
                    outputs.push(result);
                }
                Instruction::Bdv => {
                    self.reg_b = self.reg_a >> combo_operand;
                }
                Instruction::Cdv => {
                    self.reg_c = self.reg_a >> combo_operand;
                }
            }

            instruction_pointer += 2;
        }

        outputs
    }

    fn combo_operand(&self, num: u8) -> usize {
        match num {
            x @ 0..=3 => x as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_val(val: u8) -> Option<Self> {
        match val {
            0 => Some(Instruction::Adv),
            1 => Some(Instruction::Bxl),
            2 => Some(Instruction::Bst),
            3 => Some(Instruction::Jnz),
            4 => Some(Instruction::Bxc),
            5 => Some(Instruction::Out),
            6 => Some(Instruction::Bdv),
            7 => Some(Instruction::Cdv),
            _ => None,
        }
    }
}
