use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let xword = parse_xword("input/day04.txt")?;

    println!("Part1: {}", part1(&xword));
    println!("Part2: {}", part2(&xword));

    Ok(())
}

fn parse_xword(file_name: &str) -> std::io::Result<Vec<Vec<char>>> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut xword = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let v = line.chars().collect::<Vec<char>>();

        xword.push(v);
    }

    Ok(xword)
}

fn part1(xword: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (i, line) in xword.iter().enumerate() {
        result += count_xmas(&String::from_iter(line.clone().into_iter()));
        result += count_xmas(&diag_string(0, i, true, &xword));
        result += count_xmas(&diag_string(0, i, false, &xword));
    }
    for j in 1..xword[0].len() {
        result += count_xmas(&diag_string(j, 0, true, &xword));
        result += count_xmas(&diag_string(j, xword.len() - 1, false, &xword));
    }
    for i in 0..xword[0].len() {
        result += count_xmas(&vertical_string(i, &xword));
    }

    result
}

fn part2(xword: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 1..(xword[0].len() - 1) {
        for j in 1..(xword.len() - 1) {
            if xword[j][i] == 'A' && check_for_x(i, j, &xword) {
                result += 1;
            }
        }
    }

    result
}

fn check_for_x(x: usize, y: usize, xword: &Vec<Vec<char>>) -> bool {
    if x == 0 || x == xword[0].len() - 1 {
        return false;
    }
    if y == 0 || y == xword.len() - 1 {
        return false;
    }
    match (xword[y - 1][x - 1], xword[y + 1][x + 1]) {
        ('M', 'S') => {}
        ('S', 'M') => {}
        _ => return false,
    }
    match (xword[y + 1][x - 1], xword[y - 1][x + 1]) {
        ('M', 'S') => {}
        ('S', 'M') => {}
        _ => return false,
    }

    true
}

fn vertical_string(x: usize, xword: &Vec<Vec<char>>) -> String {
    xword.iter().fold(String::new(), |mut acc, line| {
        acc.push(line[x]);
        acc
    })
}

fn diag_string(x: usize, y: usize, descend: bool, xword: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    if descend {
        for (i, j) in (x..xword[0].len()).zip(y..xword.len()) {
            result.push(xword[j][i]);
        }
    } else {
        for (i, j) in (x..xword[0].len()).zip((0..=y).rev()) {
            result.push(xword[j][i]);
        }
    }
    result
}

fn count_xmas(word: &str) -> usize {
    word.match_indices("XMAS").count() + word.match_indices("SAMX").count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let xword = parse_xword("../test_input/day04test.txt")?;
        let part1 = part1(&xword);
        assert_eq!(part1, 18);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let xword = parse_xword("../test_input/day04test.txt")?;
        let part2 = part2(&xword);
        assert_eq!(part2, 9);

        Ok(())
    }
}
