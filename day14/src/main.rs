use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut map1 = Map::from_file(101, 103, "input/day14.txt")?;
    let mut map2 = Map::from_file(101, 103, "input/day14.txt")?;

    map1.run_n_seconds(100);
    let part1: usize = [0usize, 1usize, 2usize, 3usize]
        .iter()
        .map(|&quadrant| map1.robots_in_quadrant(quadrant))
        .product();

    println!("part1: {}", part1);

    let mut part2 = 0;
    // Part2 -- check for symmetry?
    // while map2.robots_in_quadrant(0) != map2.robots_in_quadrant(1)
    //     || map2.robots_in_quadrant(2) != map2.robots_in_quadrant(3)
    //     || !map2.no_overlaps() {
    //     map2.run_one_second();
    //     part2 += 1;
    // }
    // nope....check for uniqueness? yep...
    while !map2.no_overlaps() {
        map2.run_one_second();
        part2 += 1;
    }

    println!("part2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Robot {
    x_pos: i32,
    y_pos: i32,
    x_velocity: i32,
    y_velocity: i32,
}

#[derive(Debug)]
struct Map {
    x_size: usize,
    y_size: usize,
    robots: Vec<Robot>,
}

impl Map {
    fn from_file(
        x_size: usize,
        y_size: usize,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut robots = Vec::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }

            let nums = line
                .split(['p', '=', ',', ' ', 'v'])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let x_pos = nums[0];
            let y_pos = nums[1];
            let x_velocity = nums[2] as i32;
            let y_velocity = nums[3] as i32;

            robots.push(Robot {
                x_pos,
                y_pos,
                x_velocity,
                y_velocity,
            });
        }

        Ok(Self {
            x_size,
            y_size,
            robots,
        })
    }

    fn run_n_seconds(&mut self, n: usize) {
        for _ in 0..n {
            self.run_one_second();
        }
    }

    fn run_one_second(&mut self) {
        for robot in &mut self.robots {
            let mut new_x_pos = robot.x_pos + robot.x_velocity;
            let mut new_y_pos = robot.y_pos + robot.y_velocity;

            if new_x_pos >= self.x_size as i32 {
                new_x_pos -= self.x_size as i32;
            } else if new_x_pos < 0 {
                new_x_pos += self.x_size as i32;
            }

            if new_y_pos >= self.y_size as i32 {
                new_y_pos -= self.y_size as i32;
            } else if new_y_pos < 0 {
                new_y_pos += self.y_size as i32;
            }

            robot.x_pos = new_x_pos;
            robot.y_pos = new_y_pos;
        }
    }

    fn robots_in_quadrant(&self, quadrant: usize) -> usize {
        let x_mid_start = self.x_size / 2 + 1;
        let y_mid_start = self.y_size / 2 + 1;

        // ranges are inclusive
        let (x_range, y_range) = match quadrant {
            0 => (0..(self.x_size / 2 - 1), 0..(self.y_size / 2 - 1)),
            1 => (x_mid_start..self.x_size - 1, 0..(self.y_size / 2 - 1)),
            // This is not how the problem statement defines the quadrants but *shrug*
            2 => (x_mid_start..self.x_size - 1, y_mid_start..self.y_size - 1),
            3 => (0..(self.x_size / 2 - 1), y_mid_start..self.y_size - 1),
            _ => unreachable!(),
        };

        self.robots
            .iter()
            .filter(|r| {
                r.x_pos as usize >= x_range.start
                    && r.x_pos as usize <= x_range.end
                    && r.y_pos as usize >= y_range.start
                    && r.y_pos as usize <= y_range.end
            })
            .count()
    }

    #[allow(dead_code)]
    fn display(&self, quadtrants: bool) {
        for y in 0..self.y_size {
            if quadtrants && y == self.y_size / 2 {
                println!();
                continue;
            }
            for x in 0..self.x_size {
                if quadtrants && x == self.x_size / 2 {
                    print!(" ");
                    continue;
                }

                match self.robots_at(x, y) {
                    0 => print!("."),
                    n @ _ => print!("{n}"),
                }
            }
            print!("\n");
        }
    }

    fn no_overlaps(&self) -> bool {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if self.robots_at(x, y) > 1 {
                    return false;
                }
            }
        }

        true
    }

    fn robots_at(&self, x: usize, y: usize) -> usize {
        self.robots
            .iter()
            .filter(|&r| r.x_pos as usize == x && r.y_pos as usize == y)
            .count()
    }
}
