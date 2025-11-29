use itertools::Itertools;
use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut maze = Maze::from_file("input/day20.txt")?;

    let start_pathfinding = Instant::now();
    let _ = maze.shortest_nocheat();
    let start_part1 = Instant::now();
    let part1 = maze.cheats_faster_than(2, 100);
    let start_part2 = Instant::now();
    let part2 = maze.cheats_faster_than(20, 100);
    let end = Instant::now();
    println!(
        "Pathfinding took: {:?}",
        start_part1.duration_since(start_pathfinding)
    );
    println!(
        "Part1: {}, duration: {:?}",
        part1,
        start_part2.duration_since(start_part1)
    );
    println!(
        "Part2: {}, duration: {:?}",
        part2,
        end.duration_since(start_part2)
    ); // 38457 too low

    // I could cache the results for part1 and use it for part2 and do it all in one pass

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
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
struct Next {
    loc: Position,
    cost: usize,
    cheated: Option<(Position, Position)>,
}

impl Eq for Next {}

impl PartialEq<Self> for Next {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.loc == other.loc
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

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    visited: HashMap<Position, usize>, // store minimum cost to get there with visited tiles
    start: Position,
    end: Position,
}

impl Maze {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut end = Position { x: 0, y: 0 };
        let mut start = Position { x: 0, y: 0 };

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
                };
                match c {
                    '#' => _ = walls.insert(p),
                    'E' => end = p,
                    'S' => start = p,
                    _ => {}
                }
            }
            height = max(height, y + 1);
        }

        Ok(Self {
            width,
            height,
            walls,
            visited: HashMap::new(),
            start,
            end,
        })
    }

    fn cheats_faster_than(&self, max_cheat_len: usize, faster_by: i32) -> usize {
        let mut count = 0;
        for combination in self.visited.keys().combinations(2) {
            if let Some(savings) = self.cheatable(combination[0], combination[1], max_cheat_len) {
                if savings as i32 >= faster_by {
                    // println!("Cheat from {:?} to {:?} saves {}", combination[0], combination[1], savings);
                    count += 1;
                }
            }
        }

        count
    }

    // Checks if two points have a valid cheat path between them. If so, returns Some of the time savings
    // by doing so. Otherwise, returns none
    fn cheatable(&self, start: &Position, end: &Position, max_cheat_len: usize) -> Option<usize> {
        if !self.visited.contains_key(start) || !self.visited.contains_key(end) {
            return None;
        }

        let distance = self.manhattan_distance(start, end);

        if distance > max_cheat_len {
            return None;
        }

        Some(
            self.visited
                .get(end)
                .unwrap()
                .abs_diff(*self.visited.get(start).unwrap())
                - distance,
        )
    }

    fn manhattan_distance(&self, start: &Position, end: &Position) -> usize {
        if !self.is_valid(start) || !self.is_valid(end) {
            panic!();
        }
        ((start.x - end.x).abs() + (start.y - end.y).abs()) as usize
    }

    fn shortest_nocheat(&mut self) -> usize {
        self.visited.insert(self.start.clone(), 0);
        let mut current = Next {
            loc: self.visited.keys().nth(0).unwrap().clone(),
            cost: 0,
            cheated: None,
        };

        let mut fringe = VecDeque::new();
        fringe.push_back(current);

        let mut shortest_nocheat = usize::MAX;

        while !fringe.is_empty() {
            current = fringe.pop_front().unwrap();
            self.visited.insert(current.loc, current.cost);

            if current.loc == self.end && current.cost < shortest_nocheat {
                shortest_nocheat = current.cost;
            }

            for neighbor in self.next_options(&current.loc) {
                if self.visited.contains_key(&neighbor) {
                    continue;
                }

                fringe.push_back(Next {
                    loc: neighbor,
                    cost: current.cost + 1,
                    cheated: current.cheated,
                });
            }
        }

        shortest_nocheat
    }

    fn next_options(&self, pos: &Position) -> Vec<Position> {
        [
            Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Position {
                x: pos.x,
                y: pos.y + 1,
            },
        ]
        .into_iter()
        .filter(|p| !self.is_wall(p))
        // .filter(|p| !self.visited.contains(p))  // Need to revisit to find all paths
        .collect::<Vec<Position>>()
    }

    fn is_valid(&self, p: &Position) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width as i32 && p.y < self.height as i32
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
                };
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                if p == self.start {
                    print!("S");
                    continue;
                }
                if p == self.end {
                    print!("E");
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
    fn test_part1() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day20test.txt")?;
        let _ = maze.shortest_nocheat();
        assert_eq!(maze.cheats_faster_than(2, 1), 44);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let mut maze = Maze::from_file("../test_input/day20test.txt")?;
        let _ = maze.shortest_nocheat();
        assert_eq!(maze.cheats_faster_than(20, 50), 285);

        Ok(())
    }
}
