use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day03.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines();

    let re = Regex::new(r"mul\((?P<num0>\d{1,3}),(?P<num1>\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut en = true;

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }

        for cap in re.captures_iter(&line) {
            match cap.get(0).unwrap().as_str() {
                "do()" => en = true,
                "don't()" => en = false,
                _ => {
                    let num0 = cap["num0"].parse::<i32>().unwrap();
                    let num1 = cap["num1"].parse::<i32>().unwrap();
                    part1_sum += num0 * num1;
                    if en {
                        part2_sum += num0 * num1;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);

    Ok(())
}
