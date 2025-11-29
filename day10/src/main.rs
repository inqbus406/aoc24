use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() -> std::io::Result<()> {
    let map = Map::from_file("input/day10.txt")?;

    let part1 = map.part1();
    let part2 = map.part2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

struct Map {
    map: Vec<Vec<u32>>,
    trailheads: Vec<Point>,
}

impl Map {
    fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut map = Vec::new();
        let mut trailheads = Vec::new();

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            for (x, c) in line.chars().enumerate() {
                let num = c.to_digit(10).unwrap();
                if num == 0 {
                    trailheads.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }

        Ok(Self { map, trailheads })
    }

    fn part1(&self) -> usize {
        self.trailheads
            .iter()
            .map(|p| self.explore(p).iter().count())
            .sum()
    }

    fn part2(&self) -> usize {
        self.trailheads.iter().map(|p| self.explore_part2(p)).sum()
    }

    fn passable(&self, point0: &Point, point1: &Point) -> bool {
        self.lookup(point1) == self.lookup(point0) + 1
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x < self.map[0].len() as i32
            && point.y >= 0
            && point.y < self.map.len() as i32
    }

    fn lookup(&self, point: &Point) -> u32 {
        if !self.in_bounds(point) {
            // Out of bounds
            panic!();
        }
        self.map[point.y as usize][point.x as usize]
    }

    // Returns 9s reachable via this starting point
    fn explore(&self, start: &Point) -> HashSet<Point> {
        if !self.in_bounds(start) {
            // Out of bounds
            return HashSet::new();
        }
        if self.lookup(start) == 9 {
            let mut result = HashSet::new();
            result.insert(start.clone());
            return result;
        }

        self.get_neighbors(start)
            .into_iter()
            .filter(|p| self.passable(start, p))
            .flat_map(|p| self.explore(&p))
            .collect::<HashSet<_>>()
    }

    fn explore_part2(&self, start: &Point) -> usize {
        if !self.in_bounds(start) {
            // Out of bounds
            return 0;
        }
        if self.lookup(start) == 9 {
            return 1;
        }
        self.get_neighbors(start)
            .into_iter()
            .filter(|p| self.passable(start, p))
            .map(|p| self.explore_part2(&p))
            .sum()
    }

    fn get_neighbors(&self, start: &Point) -> Vec<Point> {
        vec![
            Point {
                x: start.x + 1,
                y: start.y,
            },
            Point {
                x: start.x - 1,
                y: start.y,
            },
            Point {
                x: start.x,
                y: start.y - 1,
            },
            Point {
                x: start.x,
                y: start.y + 1,
            },
        ]
        .into_iter()
        .filter(|p| self.in_bounds(p))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let map = Map::from_file("../test_input/day10test.txt")?;
        assert_eq!(map.part1(), 36);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let map = Map::from_file("../test_input/day10test.txt")?;
        assert_eq!(map.part2(), 81);

        Ok(())
    }
}
