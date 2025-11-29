use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day19.txt")?;
    let reader = BufReader::new(f);

    let (towels, patterns) = get_towels_and_patterns(reader);

    let start = Instant::now();
    let part1 = patterns.iter().filter(|p| is_possible(p, &towels)).count();
    let part1_end = Instant::now();
    println!(
        "Part1: {}, duration: {:?}",
        part1,
        part1_end.duration_since(start)
    );

    let part2_start = Instant::now();
    let mut cached_searcher = Searcher::new();

    let part2 = patterns
        .iter()
        .map(|p| cached_searcher.possibilities(p, &towels))
        .sum::<usize>();
    let part2_end = Instant::now();
    println!(
        "Part2: {}, duration: {:?}",
        part2,
        part2_end.duration_since(part2_start)
    );

    Ok(())
}

fn get_towels_and_patterns(reader: BufReader<File>) -> (Vec<String>, Vec<String>) {
    let mut lines = reader.lines();
    let towels = lines
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .map(|s| String::from(s))
        .collect::<Vec<_>>();

    let mut patterns = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }

        patterns.push(line);
    }

    (towels, patterns)
}

fn is_possible(pattern: &str, towels: &Vec<String>) -> bool {
    if towels.iter().any(|s| s == pattern) {
        return true;
    }
    for towel in towels {
        match pattern.find(towel) {
            Some(0) => {
                let substring = &pattern[towel.chars().count()..];
                if is_possible(substring, &towels) {
                    return true;
                }
            }
            _ => continue,
        }
    }

    false
}

struct Searcher<'a> {
    cache: HashMap<&'a str, usize>,
}

impl<'a> Searcher<'a> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn possibilities(&mut self, pattern: &'a str, towels: &Vec<String>) -> usize {
        if self.cache.contains_key(pattern) {
            return self.cache[pattern];
        }
        let matching_towels = towels
            .iter()
            .filter(|s| s.len() == pattern.len())
            .filter(|s| s == &pattern)
            .collect::<HashSet<_>>();
        let mut count = matching_towels.len();
        for towel in towels {
            if matching_towels.contains(&towel) {
                continue;
            }
            match pattern.find(towel) {
                Some(0) => {
                    let substring = &pattern[towel.chars().count()..];
                    count += self.possibilities(substring, towels);
                }
                _ => continue,
            }
        }

        self.cache.insert(pattern, count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let f = File::open("../test_input/day19test.txt")?;
        let reader = BufReader::new(f);
        let (towels, patterns) = get_towels_and_patterns(reader);

        let part1 = patterns.iter().filter(|p| is_possible(p, &towels)).count();
        assert_eq!(part1, 6);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let f = File::open("../test_input/day19test.txt")?;
        let reader = BufReader::new(f);
        let (towels, patterns) = get_towels_and_patterns(reader);

        let mut cached_searcher = Searcher::new();

        let part2 = patterns
            .iter()
            .map(|p| cached_searcher.possibilities(p, &towels))
            .sum::<usize>();
        assert_eq!(part2, 16);

        Ok(())
    }
}
