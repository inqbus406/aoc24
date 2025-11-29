use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut maze = Maze::from_file("input/day16.txt")?;

    let (part1, part2) = maze.solve();
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Next {
    loc: Position,
    cost: usize,
    tiles: HashMap<Position, usize>,
}

impl Eq for Next {}

impl PartialEq<Self> for Next {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.loc == other.loc && self.tiles == other.tiles
    }
}

impl PartialOrd<Self> for Next {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Next {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
    visited: HashMap<Position, usize>, // store minimum cost to get there with visited tiles
    end: Position,
    tiles: HashMap<Position, usize>,
}

impl Maze {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut visited = HashMap::new();
        let mut end = Position {
            x: 0,
            y: 0,
            dir: Direction::None,
        };

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                let p = Position {
                    x: x as i32,
                    y: y as i32,
                    dir: Direction::None,
                };
                match c {
                    '#' => _ = walls.insert(p),
                    'E' => end = p,
                    'S' => {
                        _ = visited.insert(
                            Position {
                                x: p.x,
                                y: p.y,
                                dir: Direction::East,
                            },
                            0,
                        )
                    }
                    _ => {}
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
            tiles: HashMap::new(),
        })
    }

    fn solve(&mut self) -> (usize, usize) {
        let mut current = Next {
            loc: self.visited.keys().nth(0).unwrap().clone(),
            cost: 0,
            tiles: HashMap::new(),
        };
        current.tiles.insert(current.loc, 0);

        let mut fringe = BinaryHeap::new();
        fringe.push(current);

        let mut shortest = usize::MAX;

        while !fringe.is_empty() {
            current = fringe.pop().unwrap();
            self.visited.insert(current.loc, current.cost);

            if current.loc == self.end && current.cost <= shortest {
                if current.cost < shortest {
                    self.tiles.clear();
                    shortest = current.cost;
                }
                self.tiles.extend(&current.tiles);
            }

            for (neighbor, cost) in self.next_options(&current.loc) {
                if self.visited.contains_key(&neighbor) {
                    if *self.visited.get(&neighbor).unwrap() as i32
                        >= (current.cost + cost) as i32 - 1000
                    {
                        self.visited
                            .entry(neighbor)
                            .and_modify(|v| *v = current.cost + cost);
                    } else {
                        continue;
                    }
                }
                let mut tiles = HashMap::new();
                tiles.insert(neighbor, current.tiles.len());
                tiles.extend(&current.tiles);

                fringe.push(Next {
                    loc: neighbor,
                    cost: cost + current.cost,
                    tiles,
                });
            }
        }

        (shortest, self.tiles.len())
    }

    fn next_options(&self, pos: &Position) -> Vec<(Position, usize)> {
        let mut options = Vec::new();
        let current = pos.clone();
        let neighbors = [
            Position {
                x: current.x + 1,
                y: current.y,
                dir: Direction::East,
            },
            Position {
                x: current.x - 1,
                y: current.y,
                dir: Direction::West,
            },
            Position {
                x: current.x,
                y: current.y - 1,
                dir: Direction::North,
            },
            Position {
                x: current.x,
                y: current.y + 1,
                dir: Direction::South,
            },
        ]
        .into_iter()
        .filter(|p| !self.is_wall(p))
        // .filter(|p| !self.visited.contains(p))  // Originally used this for part1, but needed all passable neighbors for part2
        .collect::<Vec<Position>>();

        for neighbor in neighbors {
            options.push((neighbor, 1 + current.dir.turns_to(&neighbor.dir) * 1000));
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
                let p = Position {
                    x: x as i32,
                    y: y as i32,
                    dir: Direction::None,
                };
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                if self.tiles.contains_key(&p) {
                    print!("O");
                    continue;
                }
                if self.visited.contains_key(&p) {
                    print!("x");
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

    #[test]
    fn test1_part1() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day16test.txt")?;
        let (part1, _) = maze.solve();
        assert_eq!(part1, 7036);

        Ok(())
    }

    #[test]
    fn test2_part1() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day16test2.txt")?;
        let (part1, _) = maze.solve();
        assert_eq!(part1, 11048);

        Ok(())
    }

    #[test]
    fn test1_part2() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day16test.txt")?;
        let (_, part2) = maze.solve();
        assert_eq!(part2, 45);

        Ok(())
    }

    #[test]
    fn test2_part2() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day16test2.txt")?;
        let (_, part2) = maze.solve();
        assert_eq!(part2, 64);

        Ok(())
    }
}
