use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const PART2_OFFSET: i64 = 10000000000000;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day13.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let parse_start = Instant::now();

    let mut machines = Vec::new();

    let mut buffer = String::new();
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            machines.push(Machine::from_str(&buffer));
            buffer.clear();
        }
        buffer.push_str(&line);
    }
    machines.push(Machine::from_str(&buffer));

    let start = Instant::now();
    println!("File parse time: {:?}", start.duration_since(parse_start));
    let part1 = machines.iter().map(|m| m.min_tokens_to_win()).sum::<i64>();
    let part1_end = Instant::now();
    println!(
        "Part 1: {}, duration: {:?}",
        part1,
        part1_end.duration_since(start)
    );

    // part2
    for machine in machines.iter_mut() {
        machine.prize.0 += PART2_OFFSET;
        machine.prize.1 += PART2_OFFSET;
    }
    let part2 = machines.iter().map(|m| m.min_tokens_to_win()).sum::<i64>();
    let part2_end = Instant::now();
    println!(
        "Part 2: {}, duration: {:?}",
        part2,
        part2_end.duration_since(part1_end)
    );

    Ok(())
}

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let button_a_re = Regex::new(r"Button A: X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap();
        let button_b_re = Regex::new(r"Button B: X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap();
        let prize_re = Regex::new(r"Prize: X=(?P<x>\d+), Y=(?P<y>\d+)").unwrap();

        let button_a_x = button_a_re.captures(s).unwrap()["x"]
            .parse::<i64>()
            .unwrap();
        let button_a_y = button_a_re.captures(s).unwrap()["y"]
            .parse::<i64>()
            .unwrap();
        let button_b_x = button_b_re.captures(s).unwrap()["x"]
            .parse::<i64>()
            .unwrap();
        let button_b_y = button_b_re.captures(s).unwrap()["y"]
            .parse::<i64>()
            .unwrap();
        let prize_x = prize_re.captures(s).unwrap()["x"].parse::<i64>().unwrap();
        let prize_y = prize_re.captures(s).unwrap()["y"].parse::<i64>().unwrap();

        Self {
            button_a: (button_a_x, button_a_y),
            button_b: (button_b_x, button_b_y),
            prize: (prize_x, prize_y),
        }
    }

    fn min_tokens_to_win(&self) -> i64 {
        let ax = self.button_a.0;
        let ay = self.button_a.1;
        let bx = self.button_b.0;
        let by = self.button_b.1;
        let zx = self.prize.0;
        let zy = self.prize.1;

        let b_presses = ((ay * zx) - (ax * zy)) / ((bx * ay) - (ax * by));
        let a_presses = (zx - bx * b_presses) / ax;

        let result = (
            a_presses * ax + b_presses * bx,
            a_presses * ay + b_presses * by,
        );

        if result.0 != self.prize.0 || result.1 != self.prize.1 {
            0
        } else {
            a_presses * 3 + b_presses
        }
    }
}
