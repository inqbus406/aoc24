use itertools::{repeat_n, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day22.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut sum = 0;
    let mut monkeys = Vec::new();

    for line in lines {
        let Ok(monkey) = line else {
            continue;
        };
        if monkey.is_empty() {
            continue;
        }

        let monkey = monkey.parse::<i64>().unwrap();
        monkeys.push(monkey);
        sum += iterations(monkey, 2000);
    }

    println!("Part1: {sum}");
    println!("Part2: {}", part2(&monkeys));

    Ok(())
}

fn part2(monkeys: &Vec<i64>) -> i64 {
    let mut buyers = Vec::new();
    for buyer in monkeys {
        let mut nums = vec![*buyer];
        for _ in 0..2000 {
            nums.push(iterations(*nums.last().unwrap(), 1));
        }
        let deltas = nums
            .windows(2)
            .map(|pair| (pair[1] % 10) - (pair[0] % 10))
            .collect::<Vec<i64>>();
        buyers.push((nums, deltas));
    }
    let mut max = 0;
    let possible_deltas = -10..=10;
    for seq in repeat_n(possible_deltas, 4).multi_cartesian_product() {
        let mut sum = 0;
        for buyer in &buyers {
            if let Some(idx) = find_first_occurrence(&buyer.1, &seq) {
                sum += buyer.0[idx] % 10;
            }
        }
        max = max.max(sum);
    }

    max
}

fn find_first_occurrence(deltas: &Vec<i64>, sequence: &Vec<i64>) -> Option<usize> {
    for (index, window) in deltas.windows(4).enumerate() {
        if window == sequence {
            return Some(index + 4);
        }
    }
    None
}

fn iterations(number: i64, n: usize) -> i64 {
    let mut secret_number = number;

    for _ in 0..n {
        secret_number = mix(secret_number, secret_number * 64);
        secret_number = prune(secret_number);

        secret_number = mix(secret_number, secret_number / 32);
        secret_number = prune(secret_number);

        secret_number = mix(secret_number, secret_number * 2048);
        secret_number = prune(secret_number);
    }

    secret_number
}

fn mix(secret_num: i64, given_num: i64) -> i64 {
    secret_num ^ given_num
}

fn prune(number: i64) -> i64 {
    number % 16777216
}
