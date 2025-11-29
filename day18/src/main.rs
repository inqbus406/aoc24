use std::collections::{HashSet, LinkedList, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // let walls = get_walls("test_input/day18test.txt")?;
    let walls = get_walls("input/day18.txt")?;
    let maze = Maze::from_slice(71, 71, &walls[0..1024]);
    // let maze = Maze::from_slice(7, 7, &walls[0..12]);
    // let maze = Maze::from_file(7, 7, 12, "test_input/day18test.txt")?;
    // let maze = Maze::from_file(71, 71, 1024, "input/day18.txt")?;
    // maze.display();

    let part1 = maze.shortest_path_len().unwrap();
    println!("Part1: {}", part1);

    // Part 2
    for i in 0..walls.len() {
        let maze = Maze::from_slice(71, 71, &walls[0..i]);
        // let maze = Maze::from_slice(7, 7, &walls[0..i]);
        match maze.shortest_path_len() {
            Some(_) => {}
            None => {
                println!("Part2: {}", i);
                return Ok(());
            }
        }
        // if maze.shortest_path_len() == None {
        //     println!("Part2: {}", i);
        //     return Ok(())
        // }
        println!("i: {i} byte: {},{}", walls[i].x, walls[i].y);
    }

    Ok(())
}

fn get_walls(path: impl AsRef<Path>) -> std::io::Result<Vec<Position>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut walls = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let nums = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        walls.push(Position {
            x: nums[0],
            y: nums[1],
        });
    }

    Ok(walls)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Next {
    loc: Position,
    steps: usize,
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
}

impl Maze {
    fn from_file(
        width: usize,
        height: usize,
        lines_to_read: usize,
        path: impl AsRef<Path>,
    ) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut walls = HashSet::new();

        for (i, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            if i >= lines_to_read {
                break;
            }
            let nums = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            walls.insert(Position {
                x: nums[0],
                y: nums[1],
            });
        }

        Ok(Self {
            width,
            height,
            walls,
        })
    }

    fn from_slice(width: usize, height: usize, walls_list: &[Position]) -> Self {
        let mut walls = HashSet::new();
        walls_list
            .iter()
            .for_each(|pos| _ = walls.insert(pos.clone()));

        Self {
            width,
            height,
            walls,
        }
    }

    fn shortest_path_len(&self) -> Option<usize> {
        let mut fringe = VecDeque::new();
        let mut visited = vec![vec![false; self.width]; self.height];
        fringe.push_back(Next {
            loc: Position { x: 0, y: 0 },
            steps: 0,
        });
        let finish = Position {
            x: (self.width - 1) as i32,
            y: (self.height - 1) as i32,
        };

        while let Some(cur) = fringe.pop_front() {
            if cur.loc == finish {
                // self.display(&visited);
                return Some(cur.steps);
            }
            // dbg!(&fringe);
            // if visited.len() % 1000 == 0 {
            // dbg!(visited.len());
            // }

            visited[cur.loc.y as usize][cur.loc.x as usize] = true;
            for neighbor in self.get_neighbors(&cur.loc).iter() {
                if !visited[cur.loc.y as usize][cur.loc.x as usize] {
                    continue;
                }
                let next = Next {
                    loc: neighbor.clone(),
                    steps: cur.steps + 1,
                };
                // for n in fringe.iter().filter(|n| n.loc == next.loc) {
                //     if next.steps >= n.steps {
                //         continue;
                //     }
                // }
                if !fringe.iter().any(|n| n.loc == next.loc) {
                    fringe.push_back(next);
                }
            }
            // self.get_neighbors(&cur.loc).iter()
            //     .filter(|n| !visited[n.y as usize][n.x as usize])
            //     .for_each(|n| fringe.push_back(Next {loc: *n, steps: cur.steps + 1}));
        }

        None
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
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
        .filter(|p| self.is_valid(p))
        .filter(|p| !self.walls.contains(p))
        .collect()
    }

    fn is_valid(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Position {
                    x: x as i32,
                    y: y as i32,
                };
                if self.walls.contains(&p) {
                    print!("#");
                    continue;
                }
                // if visited.contains(&p) {
                //     print!("O");
                //     continue;
                // }
                print!(".");
            }
            println!();
        }
    }
}
