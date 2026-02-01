use cached::proc_macro::cached;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let dir = if args.contains(&String::from("--test")) {
        PathBuf::from("test_input")
    } else {
        PathBuf::from("input")
    };
    let f = File::open(dir.join("day21.txt"))?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in lines {
        let Ok(code) = line else {
            continue;
        };
        if code.is_empty() {
            continue;
        }
        let mut keypad_robot = NumericKeypadRobot::new();
        let part1_length = code
            .chars()
            .map(|c| keypad_robot.enter_digit(&NumericKey::from_char(c), 2))
            .sum::<usize>();
        keypad_robot.reset();

        let part2_length = code
            .chars()
            .map(|c| keypad_robot.enter_digit(&NumericKey::from_char(c), 25))
            .sum::<usize>();

        let num = code
            .split('A')
            .take(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        part1_sum += num[0] * part1_length;
        part2_sum += num[0] * part2_length;
        // println!("{}: len: {}", code, part1_length);
    }

    println!("Part1: {part1_sum}");
    println!("Part2: {part2_sum}");

    Ok(())
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum NumericKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumericKey {
    fn from_char(c: char) -> Self {
        match c {
            'A' => NumericKey::A,
            '0' => NumericKey::Zero,
            '1' => NumericKey::One,
            '2' => NumericKey::Two,
            '3' => NumericKey::Three,
            '4' => NumericKey::Four,
            '5' => NumericKey::Five,
            '6' => NumericKey::Six,
            '7' => NumericKey::Seven,
            '8' => NumericKey::Eight,
            '9' => NumericKey::Nine,
            _ => unreachable!(),
        }
    }

    fn position(&self) -> (usize, usize) {
        match self {
            NumericKey::A => (2, 3),
            NumericKey::Zero => (1, 3),
            NumericKey::One => (0, 2),
            NumericKey::Two => (1, 2),
            NumericKey::Three => (2, 2),
            NumericKey::Four => (0, 1),
            NumericKey::Five => (1, 1),
            NumericKey::Six => (2, 1),
            NumericKey::Seven => (0, 0),
            NumericKey::Eight => (1, 0),
            NumericKey::Nine => (2, 0),
        }
    }

    fn from_position(pos: (i32, i32)) -> Option<Self> {
        match pos {
            (2, 3) => Some(NumericKey::A),
            (1, 3) => Some(NumericKey::Zero),
            (0, 2) => Some(NumericKey::One),
            (1, 2) => Some(NumericKey::Two),
            (2, 2) => Some(NumericKey::Three),
            (0, 1) => Some(NumericKey::Four),
            (1, 1) => Some(NumericKey::Five),
            (2, 1) => Some(NumericKey::Six),
            (0, 0) => Some(NumericKey::Seven),
            (1, 0) => Some(NumericKey::Eight),
            (2, 0) => Some(NumericKey::Nine),
            _ => None,
        }
    }

    fn next(&self, dir: &TClusterKey) -> Option<Self> {
        let x = self.position().0 as i32;
        let y = self.position().1 as i32;
        match dir {
            TClusterKey::Down => Self::from_position((x, y + 1)),
            TClusterKey::Up => Self::from_position((x, y - 1)),
            TClusterKey::Left => Self::from_position((x - 1, y)),
            TClusterKey::Right => Self::from_position((x + 1, y)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum TClusterKey {
    A,
    Right,
    Down,
    Left,
    Up,
}

impl TClusterKey {
    fn position(&self) -> (usize, usize) {
        match self {
            TClusterKey::A => (2, 0),
            TClusterKey::Right => (2, 1),
            TClusterKey::Down => (1, 1),
            TClusterKey::Left => (0, 1),
            TClusterKey::Up => (1, 0),
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            TClusterKey::A => 'A',
            TClusterKey::Right => '>',
            TClusterKey::Down => 'v',
            TClusterKey::Left => '<',
            TClusterKey::Up => '^',
        }
    }
}

#[derive(Debug, Clone)]
struct Next {
    pos: NumericKey,
    path: Vec<TClusterKey>,
}

struct NumericKeypadRobot {
    position: NumericKey,
}

impl NumericKeypadRobot {
    fn new() -> Self {
        Self {
            position: NumericKey::A,
        }
    }

    fn reset(&mut self) {
        self.position = NumericKey::A;
    }

    fn enter_digit(&mut self, digit: &NumericKey, n_robots: usize) -> usize {
        let mut min_length = usize::MAX;

        for path in self.moves_to_digit(digit) {
            let mut temp_length = 0;
            let mut current_key = TClusterKey::A;
            for key in path {
                temp_length += enter_direction_length(current_key, key, n_robots);
                current_key = key;
            }
            if temp_length < min_length {
                min_length = temp_length;
            }
        }
        self.position = digit.clone();

        min_length
    }

    fn moves_to_digit(&self, digit: &NumericKey) -> Vec<Vec<TClusterKey>> {
        // let start = self.position.position();
        let end = digit.position();

        let mut result = Vec::new();

        let mut fringe = VecDeque::new();
        fringe.push_back(Next {
            pos: self.position.clone(),
            path: Vec::new(),
        });

        while let Some(current) = fringe.pop_front() {
            // dbg!(&current);
            if current.pos.position() == end {
                let mut good_path = current.path.clone();
                good_path.push(TClusterKey::A);
                result.push(good_path);
            }

            for next in Self::get_moves_toward(&current.pos, digit) {
                // dbg!(&next);
                let mut path = current.path.clone();
                path.push(next.clone());
                fringe.push_back(Next {
                    pos: current.pos.next(&next).unwrap(),
                    path,
                });
            }
        }

        result
    }

    fn get_moves_toward(start: &NumericKey, end: &NumericKey) -> Vec<TClusterKey> {
        let result = [
            TClusterKey::Up,
            TClusterKey::Down,
            TClusterKey::Left,
            TClusterKey::Right,
        ]
        .into_iter()
        .filter(|pos| start.next(&pos).is_some())
        .filter(|pos| start.next(&pos).unwrap().position() != (0, 3)) // Don't step in the gap
        .filter(|pos| {
            Self::manhattan_distance(&start.next(&pos).unwrap(), end)
                < Self::manhattan_distance(start, end)
        })
        .collect();

        result
    }

    fn manhattan_distance(start: &NumericKey, end: &NumericKey) -> usize {
        start.position().0.abs_diff(end.position().0)
            + start.position().1.abs_diff(end.position().1)
    }
}

#[cached]
fn enter_direction_length(start: TClusterKey, end: TClusterKey, n_robots: usize) -> usize {
    if n_robots == 1 {
        return get_all_moves_to_key(start, end)[0].len();
    }

    let mut min_length = usize::MAX;

    for path in get_all_moves_to_key(start, end) {
        let mut total_length = 0;
        let mut current_pos = TClusterKey::A;

        for target_key in path {
            total_length += enter_direction_length(current_pos, target_key, n_robots - 1);
            current_pos = target_key;
        }

        min_length = min_length.min(total_length);
    }

    min_length
}

#[cached]
fn get_all_moves_to_key(start: TClusterKey, end: TClusterKey) -> Vec<Vec<TClusterKey>> {
    if start == end {
        return vec![vec![TClusterKey::A]];
    }

    let start_pos = start.position();
    let end_pos = end.position();
    let dx = end_pos.0 as i32 - start_pos.0 as i32;
    let dy = end_pos.1 as i32 - start_pos.1 as i32;

    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();

    for _ in 0..dx.abs() {
        if dx > 0 {
            horizontal.push(TClusterKey::Right);
        } else {
            horizontal.push(TClusterKey::Left);
        }
    }

    for _ in 0..dy.abs() {
        if dy > 0 {
            vertical.push(TClusterKey::Down);
        } else {
            vertical.push(TClusterKey::Up);
        }
    }

    let mut paths = Vec::new();

    // Try horizontal first, then vertical (if valid)
    if !(start_pos.1 == 0 && end_pos == (0, 1)) {
        // Avoid gap
        let mut path1 = horizontal.clone();
        path1.extend(vertical.clone());
        path1.push(TClusterKey::A);
        paths.push(path1);
    }

    // Try vertical first, then horizontal (if valid and different)
    if !(start_pos == (0, 1) && end_pos.1 == 0) && !vertical.is_empty() && !horizontal.is_empty() {
        let mut path2 = vertical.clone();
        path2.extend(horizontal.clone());
        path2.push(TClusterKey::A);
        paths.push(path2);
    }

    // If only one direction needed or gap avoidance left us with one path
    if paths.is_empty() {
        let mut path = horizontal;
        path.extend(vertical);
        path.push(TClusterKey::A);
        paths.push(path);
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            NumericKeypadRobot::manhattan_distance(&NumericKey::A, &NumericKey::Zero),
            1
        );
        assert_eq!(
            NumericKeypadRobot::manhattan_distance(&NumericKey::A, &NumericKey::One),
            3
        );
        assert_eq!(
            NumericKeypadRobot::manhattan_distance(&NumericKey::A, &NumericKey::Seven),
            5
        );
    }
}
