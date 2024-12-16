use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut maze = Maze::from_file("input/day16.txt")?;

    // dbg!(&maze);
    // maze.display();
    // return Ok(());
    let part1 = maze.solve();
    println!("Part1: {}", part1);

    Ok(())
}

struct Next(Position, usize);

impl Eq for Next {}

impl PartialEq<Self> for Next {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd<Self> for Next {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Next {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    // Get a number value associated with each direction to calculate angular offset
    fn value(&self) -> u8 {
        match self {
            Direction::North => 1,
            Direction::East => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::None => panic!(),
        }
    }

    fn turns_to(&self, other: &Direction) -> usize {
        let difference = self.value().abs_diff(other.value());
        if difference == 2 {
            return 2;
        }
        (difference % 2) as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl PartialEq for Position {
    // Don't compare direction for same position
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    visited: HashSet<Position>,
    end: Position,
    score: usize,
}

impl Maze {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut visited = HashSet::new();
        let mut end = Position { x: 0, y: 0, dir: Direction::None };

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                let p = Position { x: x as i32, y: y as i32, dir: Direction::None };
                match c {
                    '#' => _ = walls.insert(p),
                    'E' => end = p,
                    'S' => _ = visited.insert(Position{x: p.x, y: p.y, dir: Direction::East}),
                    _   => {}
                }
            }
            height = max(height, y + 1);
        }

        Ok(Self {
            width,
            height,
            walls,
            visited,
            end,
            score: 0,
        })
    }

    fn solve(&mut self) -> usize {
        let mut current = Next(self.visited.iter().last().unwrap().clone(), 0);
        let mut fringe = BinaryHeap::new();
        fringe.push(current);

        let mut shortest = usize::MAX;

        while !fringe.is_empty() {
            current = fringe.pop().unwrap();
            self.visited.insert(current.0);
            if current.0 == self.end {
                shortest = min(shortest, current.1);
            }

            for (neighbor, cost) in self.next_options(&current.0) {
                fringe.push(Next(neighbor, cost + current.1));
            }
        }

        shortest

        // self.next_options().iter()
        //     .map(|option| option.solve())
        //     .min()
        //     .unwrap_or(usize::MAX)
    }

    fn next_options(&self, pos: &Position) -> Vec<(Position, usize)> {
        let mut options = Vec::new();
        let current = pos.clone();
        let neighbors = [Position { x: current.x + 1, y: current.y, dir: Direction::East },
                                    Position { x: current.x - 1, y: current.y, dir: Direction::West },
                                    Position { x: current.x, y: current.y - 1, dir: Direction::North },
                                    Position { x: current.x, y: current.y + 1, dir: Direction::South }]
            .into_iter().filter(|p| !self.is_wall(p))
            .filter(|p| !self.visited.contains(p))
            .collect::<Vec<Position>>();

        for neighbor in neighbors {
            options.push((neighbor, self.score + 1 + current.dir.turns_to(&neighbor.dir) * 1000));
        }

        options
    }

    fn is_wall(&self, p: &Position) -> bool {
        self.walls.contains(&p)
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Position { x: x as i32, y: y as i32, dir: Direction::None };
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns_to() {
        assert_eq!(Direction::North.turns_to(&Direction::North), 0);
        assert_eq!(Direction::North.turns_to(&Direction::East), 1);
        assert_eq!(Direction::North.turns_to(&Direction::South), 2);
        assert_eq!(Direction::North.turns_to(&Direction::West), 1);
        assert_eq!(Direction::East.turns_to(&Direction::West), 2);
        assert_eq!(Direction::East.turns_to(&Direction::South), 1);
        assert_eq!(Direction::East.turns_to(&Direction::North), 1);
    }
}