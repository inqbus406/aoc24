use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Position = (i32, i32);

fn main() -> std::io::Result<()> {
    let mut map = Map::from_file("input/day06.txt")?;
    let initial_guard_pos = map.guard_pos;

    while map.move_guard() {}

    println!("Part1: {}", part1(&mut map));
    println!("Part2: {}", part2(&map, initial_guard_pos));

    Ok(())
}

fn part1(map: &mut Map) -> usize {
    while map.move_guard() {}

    map.visited.len()
}

fn part2(map: &Map, start_pos: Position) -> usize {
    let mut works = HashSet::new();

    for (x, y) in map.visited.iter() {
        let mut test_obstacles = map.obstacles.clone();
        test_obstacles.insert((*x, *y));
        let mut m = Map::new(map.x_size, map.y_size, start_pos, &test_obstacles);
        if m.check_for_loop(map.visited.len() * 2) {
            works.insert((*x, *y));
        }
    }

    works.len()
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
        match &self {
            Direction::North => *self = Self::East,
            Direction::East => *self = Self::South,
            Direction::South => *self = Self::West,
            Direction::West => *self = Self::North,
        }
    }
}

#[derive(Debug)]
struct Map {
    guard_pos: Position,
    guard_dir: Direction,
    x_size: usize,
    y_size: usize,
    obstacles: HashSet<Position>,
    visited: HashSet<Position>,
}

impl Map {
    fn new(
        x_size: usize,
        y_size: usize,
        guard_pos: Position,
        obstacles: &HashSet<Position>,
    ) -> Self {
        let mut visited = HashSet::new();
        visited.insert(guard_pos);

        Self {
            guard_pos,
            guard_dir: Direction::North,
            x_size,
            y_size,
            obstacles: obstacles.clone(),
            visited,
        }
    }

    fn from_file(path: &str) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut obstacles: HashSet<Position> = HashSet::new();
        let mut guard_position: Position = (0, 0);
        let mut x_size = 0;
        let mut y_size = 0;

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            for (x, c) in line.chars().enumerate() {
                x_size = max(x_size, x);
                match c {
                    '#' => _ = obstacles.insert((x as i32, y as i32)),
                    '^' => guard_position = (x as i32, y as i32),
                    _ => continue,
                }
            }
            y_size = max(y_size, y);
        }

        Ok(Self::new(
            x_size + 1,
            y_size + 1,
            guard_position,
            &obstacles,
        ))
    }

    fn move_guard(&mut self) -> bool {
        let pos_update = match self.guard_dir {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };
        let next_pos = (
            self.guard_pos.0 + pos_update.0,
            self.guard_pos.1 + pos_update.1,
        );
        if next_pos.0 < 0
            || next_pos.0 >= self.x_size as i32
            || next_pos.1 < 0
            || next_pos.1 >= self.y_size as i32
        {
            return false;
        }
        if self.obstacles.contains(&next_pos) {
            self.guard_dir.turn_right();
            return self.move_guard();
        }
        self.visited.insert(next_pos);
        self.guard_pos = next_pos;
        true
    }

    fn check_for_loop(&mut self, max_iter: usize) -> bool {
        let mut last_visited = 1;
        let mut count = 0;
        while self.move_guard() {
            count += 1;
            if self.visited.len() == last_visited && count > max_iter {
                return true;
            }
            last_visited = self.visited.len();
        }

        false
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let mut map = Map::from_file("../test_input/day06test.txt")?;
        assert_eq!(part1(&mut map), 41);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let mut map = Map::from_file("../test_input/day06test.txt")?;
        let init_guard_pos = map.guard_pos;
        part1(&mut map);
        assert_eq!(part2(&mut map, init_guard_pos), 6);

        Ok(())
    }
}
