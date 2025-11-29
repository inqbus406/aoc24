use std::fs::File;
use std::io::{BufRead, BufReader};

struct Equation(u64, Vec<u64>);

fn main() -> std::io::Result<()> {
    let equations = parse_equations("input/day07.txt")?;

    let part1_start = std::time::Instant::now();
    let part1 = equations
        .iter()
        .filter(|&e| is_valid(e, false))
        .map(|e| e.0)
        .sum::<u64>();
    let part2_start = std::time::Instant::now();
    let part2 = equations
        .iter()
        .filter(|&e| is_valid(e, true))
        .map(|e| e.0)
        .sum::<u64>();
    let end = std::time::Instant::now();
    println!(
        "Part1 = {}, duration: {:?}",
        part1,
        part2_start.duration_since(part1_start)
    );
    println!(
        "Part2 = {}, duration: {:?}",
        part2,
        end.duration_since(part2_start)
    );
    println!("Total: {:?}", end.duration_since(part1_start));

    Ok(())
}

fn parse_equations(path: &str) -> std::io::Result<Vec<Equation>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut equations = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let line = line.trim();
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let test_val = parts[0][0..parts[0].len() - 1].parse::<u64>().unwrap();
        let numbers = parts[1..]
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        equations.push(Equation(test_val, numbers));
    }

    Ok(equations)
}

fn is_valid(equation: &Equation, part2: bool) -> bool {
    match equation.1.as_slice() {
        [num] => *num == equation.0,
        [num1, num2] => {
            equation.0 == num1 + num2
                || equation.0 == num1 * num2
                || (part2 && equation.0 == concat(*num1, *num2))
        }
        [num1, num2, rest @ ..] => {
            let mut vec1 = vec![num1 + num2];
            let mut vec2 = vec![num1 * num2];
            let mut vec3 = vec![concat(*num1, *num2)];
            vec1.extend(rest);
            vec2.extend(rest);
            vec3.extend(rest);

            is_valid(&Equation(equation.0, vec1), part2)
                || is_valid(&Equation(equation.0, vec2), part2)
                || (part2 && is_valid(&Equation(equation.0, vec3), part2))
        }
        _ => false,
    }
}

fn concat(num1: u64, num2: u64) -> u64 {
    num1 * 10_u64.pow(num2.to_string().len() as u32) + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(11, 12), 1112);
        assert_eq!(concat(123, 45), 12345);
    }

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let equations = parse_equations("../test_input/day07test.txt")?;
        let part1 = equations
            .iter()
            .filter(|&e| is_valid(e, false))
            .map(|e| e.0)
            .sum::<u64>();
        assert_eq!(part1, 3749);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let equations = parse_equations("../test_input/day07test.txt")?;
        let part2 = equations
            .iter()
            .filter(|&e| is_valid(e, true))
            .map(|e| e.0)
            .sum::<u64>();
        assert_eq!(part2, 11387);

        Ok(())
    }
}
