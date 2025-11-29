use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut stones = Stones::from_file("input/day11.txt")?;

    for _ in 0..25 {
        stones.update();
    }
    let part1: u64 = stones.stones.values().sum();

    for _ in 0..50 {
        stones.update();
    }
    let part2: u64 = stones.stones.values().sum();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Stones {
    stones: HashMap<u64, u64>,
}

impl Stones {
    fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut stones = HashMap::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }

            for s in line.trim().split_whitespace() {
                *stones.entry(s.parse().unwrap()).or_insert(0) += 1;
            }
        }

        Ok(Self { stones })
    }

    fn update(&mut self) {
        let mut result = HashMap::new();
        for entry in &self.stones {
            for new_entry in Self::update_one_num(*entry.0) {
                *result.entry(new_entry.0).or_insert(0) += new_entry.1 * entry.1;
            }
        }

        self.stones = result;
    }

    fn update_one_num(num: u64) -> HashMap<u64, u64> {
        let mut result = HashMap::new();
        if num == 0 {
            result.insert(1, 1);
            return result;
        }
        let num_string = num.to_string();
        if num_string.len() % 2 == 0 {
            result.insert(
                num_string[0..num_string.len() / 2].parse::<u64>().unwrap(),
                1,
            );
            *result
                .entry(num_string[num_string.len() / 2..].parse::<u64>().unwrap())
                .or_insert(0) += 1;
            return result;
        }
        result.insert(num * 2024, 1);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let mut stones = Stones::from_file("../test_input/day11test2.txt")?;
        for _ in 0..25 {
            stones.update();
        }
        assert_eq!(stones.stones.values().sum::<u64>(), 55312);

        Ok(())
    }
}
