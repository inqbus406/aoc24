use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let (mut left_list, mut right_list) = parse_lists_from_file("input/day01.txt")?;
    left_list.sort();
    right_list.sort();

    println!("Part1 sum: {}", part1(&left_list, &right_list));
    println!("Part2 sum: {}", part2(&left_list, &right_list));

    Ok(())
}

fn parse_lists_from_file(fname: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let lines = reader.lines();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    }

    Ok((left_list, right_list))
}

fn part1(left_list: &[i32], right_list: &[i32]) -> i32 {
    left_list
        .iter()
        .zip(right_list)
        .fold(0, |acc, (&left, &right)| acc + (left - right).abs())
}

fn part2(left_list: &[i32], right_list: &[i32]) -> usize {
    let mut sum = 0;
    for num in left_list {
        sum += *num as usize * right_list.iter().filter(|&num2| *num2 == *num).count();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let (mut left_list, mut right_list) = parse_lists_from_file("../test_input/day01test.txt")?;
        left_list.sort();
        right_list.sort();
        assert_eq!(part1(&left_list, &right_list), 11);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let (left_list, right_list) = parse_lists_from_file("../test_input/day01test.txt")?;
        assert_eq!(part2(&left_list, &right_list), 31);

        Ok(())
    }
}
