use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let reports = parse_reports("input/day02.txt")?;

    let part1 = reports
        .iter()
        .filter(|&report| check_report(report))
        .count();
    let part2 = reports
        .iter()
        .filter(|&report| check_report_part2(report))
        .count();

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}

fn parse_reports(fname: &str) -> std::io::Result<Vec<Vec<i32>>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut reports = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let report = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        reports.push(report);
    }

    Ok(reports)
}

fn check_report(report: &[i32]) -> bool {
    let increasing = report[1] > report[0];

    for nums in report.windows(2).collect::<Vec<&[i32]>>() {
        match (increasing, nums[1] - nums[0]) {
            (true, 1..=3) => continue,
            (false, -3..=-1) => continue,
            _ => return false,
        }
    }

    true
}

fn check_report_part2(report: &[i32]) -> bool {
    let mut report_vec = Vec::from(report);
    if check_report(report) {
        return true;
    }

    for (i, &num) in report.iter().enumerate() {
        report_vec.remove(i);
        if check_report(&report_vec) {
            return true;
        } else {
            report_vec.insert(i, num);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let reports = parse_reports("../test_input/day02test.txt")?;
        let part1 = reports
            .iter()
            .filter(|&report| check_report(report))
            .count();
        assert_eq!(part1, 2);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let reports = parse_reports("../test_input/day02test.txt")?;
        let part2 = reports
            .iter()
            .filter(|&report| check_report_part2(report))
            .count();
        assert_eq!(part2, 4);

        Ok(())
    }
}
