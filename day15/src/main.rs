use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day15.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut map_str = String::new();

    while let Some(Ok(n)) = lines.next() {
        map_str += &format!("{}\n", &n);
        if n.is_empty() {
            break;
        }
    }

    let mut map = Map::from_str(&map_str);
    let mut moves = String::new();

    // Get the instructions
    while let Some(Ok(n)) = lines.next() {
        moves += &n;
    }

    for dir in moves.chars() {
        if dir.is_whitespace() {
            continue;
        }
        map.move_robot(dir);
    }

    println!("Part1: {}", map.gps_sum());

    // Now do part2
    // reset the map
    let mut map = Map::from_str(&map_str);
    map.part2ify();
    for dir in moves.chars() {
        if dir.is_whitespace() {
            continue;
        }
        map.move_robot(dir);
    }

    println!("Part2: {}", map.gps_sum());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    robot: Position,
    boxes: HashSet<Position>,
    walls: HashSet<Position>,
    x_size: usize,
    y_size: usize,
    doublewide: bool,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut robot = Position { x: 0, y: 0 };
        let mut x_size = 0;
        let mut y_size = 0;
        let mut boxes: HashSet<Position> = HashSet::new();
        let mut walls: HashSet<Position> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            // println!("Line {}: {}", y, line);
            for (x, c) in line.chars().enumerate() {
                let p = Position {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '@' => robot = p,
                    'O' => _ = boxes.insert(p),
                    '#' => _ = walls.insert(p),
                    _ => {}
                }

                x_size = max(x + 1, x_size);
            }
            y_size = max(y, y_size);
        }

        Self {
            robot,
            boxes,
            walls,
            x_size,
            y_size,
            doublewide: false,
        }
    }

    fn is_valid(&self, pos: &Position) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return false;
        }
        if pos.x >= self.x_size as i32 || pos.y >= self.y_size as i32 {
            return false;
        }
        true
    }

    fn is_wall(&self, pos: &Position) -> bool {
        if !self.is_valid(&pos) {
            panic!();
        }

        self.walls.contains(pos)
            || (self.doublewide
                && self.walls.contains(&Position {
                    x: pos.x - 1,
                    y: pos.y,
                }))
            || pos.x == 0
            || pos.y == 0
            || pos.x == self.x_size as i32 - 1
            || pos.y == self.y_size as i32 - 1
    }

    fn is_box(&self, pos: &Position) -> bool {
        self.boxes.contains(pos)
            || (self.doublewide
                && self.boxes.contains(&Position {
                    x: pos.x - 1,
                    y: pos.y,
                }))
    }

    fn move_robot(&mut self, dir: char) {
        let next_pos = Self::new_pos(&self.robot, dir);
        if !self.is_valid(&next_pos) {
            panic!();
        }

        if self.is_wall(&next_pos) {
            return;
        }

        // Check if there's a box there and try to move it if so
        if self.is_box(&next_pos) && !self.move_box(&next_pos, dir) {
            return;
        }

        self.robot = next_pos;
    }

    fn can_move_box(&self, b: &Position, dir: char) -> bool {
        // println!("Checking: {:?}", b);
        let orig_b = b.clone();
        let b = if self.doublewide && self.is_box(b) && !self.boxes.contains(b) {
            Position { x: b.x - 1, y: b.y }
        } else {
            orig_b
        };
        let next_pos = Self::new_pos(&b, dir);
        let next_pos_right = Position {
            x: next_pos.x + 1,
            y: next_pos.y,
        };
        if !self.is_valid(&next_pos) || (self.doublewide && !self.is_valid(&next_pos_right)) {
            panic!(); // Should never happen with there being walls
        }

        // Check if it's a wall
        if self.is_wall(&next_pos) || (self.doublewide && self.is_wall(&next_pos_right)) {
            // println!("Can't move, {:?} is a wall", &next_pos);
            return false;
        }
        if !self.is_box(&next_pos) && !self.is_box(&next_pos_right) {
            return true;
        }

        // Check if there's a box there and try to move it if so
        let right_side = if self.doublewide {
            Position { x: b.x + 1, y: b.y }
        } else {
            b.clone()
        };
        if self.is_box(&next_pos) && next_pos != right_side && !self.can_move_box(&next_pos, dir) {
            return false;
        }

        if self.doublewide
            && self.is_box(&next_pos_right)
            && next_pos_right != orig_b
            && !self.can_move_box(&next_pos_right, dir)
        {
            return false;
        }

        true
    }

    fn move_box(&mut self, b: &Position, dir: char) -> bool {
        let orig_b = b.clone();
        let b = if self.doublewide && self.is_box(b) && !self.boxes.contains(b) {
            Position { x: b.x - 1, y: b.y }
        } else {
            orig_b
        };
        let next_pos = Self::new_pos(&b, dir);
        let next_pos_right = Position {
            x: next_pos.x + 1,
            y: next_pos.y,
        };
        if !self.is_valid(&next_pos) || (self.doublewide && !self.is_valid(&next_pos_right)) {
            panic!(); // Should never happen with there being walls
        }

        // Check if it's a wall
        if self.is_wall(&next_pos) || (self.doublewide && self.is_wall(&next_pos_right)) {
            return false;
        }

        // only move if they can BOTH move!!
        let right_side = if self.doublewide {
            Position { x: b.x + 1, y: b.y }
        } else {
            b.clone()
        };
        if (self.is_box(&b) && !self.can_move_box(&b, dir))
            || (self.doublewide && self.is_box(&right_side) && !self.can_move_box(&b, dir))
        {
            return false;
        }

        // Check if there's a box there and try to move it if so
        if (next_pos != right_side && self.is_box(&next_pos) && !self.move_box(&next_pos, dir))
            || (self.doublewide
                && self.boxes.contains(&next_pos_right)
                && next_pos_right != orig_b
                && !self.move_box(&next_pos_right, dir))
        {
            return false;
        }

        self.boxes.insert(next_pos);
        self.boxes.retain(|&p| p != b);

        true
    }

    fn new_pos(p: &Position, dir: char) -> Position {
        match dir {
            '^' => Position { x: p.x, y: p.y - 1 },
            '>' => Position { x: p.x + 1, y: p.y },
            'v' => Position { x: p.x, y: p.y + 1 },
            '<' => Position { x: p.x - 1, y: p.y },
            _ => {
                println!("{dir} is not a valid direction!");
                panic!();
            }
        }
    }

    fn part2ify(&mut self) {
        self.x_size = self.x_size * 2;
        self.robot = Position {
            x: self.robot.x * 2,
            y: self.robot.y,
        };
        self.doublewide = true;

        // Move boxes and walls over to correct spot
        self.boxes = self
            .boxes
            .iter()
            .map(|p| Position { x: p.x * 2, y: p.y })
            .collect();
        self.walls = self
            .walls
            .iter()
            .map(|p| Position { x: p.x * 2, y: p.y })
            .collect();
    }

    fn gps_sum(&self) -> i32 {
        self.boxes.iter().map(|b| b.x + b.y * 100).sum()
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let p = Position {
                    x: x as i32,
                    y: y as i32,
                };
                if self.robot == p {
                    print!("@");
                    continue;
                }
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                if self.is_box(&p) {
                    print!("O");
                    continue;
                }
                print!(".");
            }
            println!();
        }
    }
}

#[allow(dead_code)]
fn run(path: impl AsRef<Path>, part2: bool) -> std::io::Result<i32> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut map_str = String::new();

    while let Some(Ok(n)) = lines.next() {
        map_str += &format!("{}\n", &n);
        if n.is_empty() {
            break;
        }
    }

    let mut map = Map::from_str(&map_str);
    if part2 {
        map.part2ify();
    }
    let mut moves = String::new();

    // Get the instructions
    while let Some(Ok(n)) = lines.next() {
        moves += &n;
    }

    for dir in moves.chars() {
        if dir.is_whitespace() {
            continue;
        }
        map.move_robot(dir);
    }

    Ok(map.gps_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() -> std::io::Result<()> {
        assert_eq!(run("../test_input/day15test1.txt", false)?, 2028);

        Ok(())
    }

    #[test]
    fn test2_part1() -> std::io::Result<()> {
        assert_eq!(run("../test_input/day15test2.txt", false)?, 10092);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        assert_eq!(run("../test_input/day15test2.txt", true)?, 9021);

        Ok(())
    }
}
