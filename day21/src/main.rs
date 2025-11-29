use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("test_input/day21test.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut part1_sum = 0;

    for line in lines {
        let Ok(code) = line else {
            continue;
        };
        if code.is_empty() {
            continue;
        }
        let mut keypad_robot = NumericKeypadRobot::new();
        let mut result = String::new();
        for c in code.chars() {
            result.push_str(&keypad_robot.enter_digit(&NumericKey::from_char(c)));
            // result.push_str(&format!("   making {}: {}      ", c, keypad_robot.enter_digit(&NumericKey::from_char(c))));
        }
        let num = code
            .split('A')
            .take(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        part1_sum += num[0] * result.len();
        println!("{}: {}, len: {}", code, result, result.len());
        println!("As: {}", result.chars().filter(|&c| c == 'A').count());
    }

    println!("Part1: {part1_sum}");

    Ok(())
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    // robot: TClusterKeypadRobot1,
}

// struct TClusterKeypadRobot1 {
//     position: TClusterKey,
//     robot: TClusterKeypadRobot2,
// }
//
// struct TClusterKeypadRobot2 {
//     position: TClusterKey,
// }

impl NumericKeypadRobot {
    fn new() -> Self {
        Self {
            position: NumericKey::A,
            // robot: TClusterKeypadRobot1::new(),
        }
    }

    fn enter_digit(&mut self, digit: &NumericKey) -> String {
        // println!("Keypad robot wants {:?}", digit);
        let mut result = String::new();
        for path in self.moves_to_digit(digit) {
            let mut temp = String::new();
            for key in path {
                // println!("Keypad robot asks for {:?}", key);
                temp.push_str(&enter_direction(&TClusterKey::A, &key, 1));
            }
            if result.is_empty() || temp.len() < result.len() {
                result = temp;
            }
        }
        self.position = digit.clone();

        // println!("Moving from {:?} to {:?}: {:?}", self.position, digit, &result);

        result
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
        // println!("Found valid paths from {:?} to {:?}", self.position, digit);
        // dbg!(&result);

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
            let test = Self::manhattan_distance(&start.next(&pos).unwrap(), end)
                < Self::manhattan_distance(start, end);
            // println!("{:?} valid move from {:?} toward {:?}? {test}", &start.next(&pos).unwrap(), start, end);
            test
        })
        .collect();

        // println!("Going from {:?} to {:?}", start, end);
        // dbg!(&result);
        result
    }

    fn manhattan_distance(start: &NumericKey, end: &NumericKey) -> usize {
        start.position().0.abs_diff(end.position().0)
            + start.position().1.abs_diff(end.position().1)
    }
}

fn enter_direction(start: &TClusterKey, end: &TClusterKey, n_robots: usize) -> String {
    let mut result = String::new();

    if n_robots == 0 {
        for key in moves_to_key(start, end) {
            // println!("TClusterKeypadRobot2 asks for {:?}", key);
            result.push(key.to_char());
        }

        return result;
    }

    for key in moves_to_key(start, end) {
        result.insert_str(0, &enter_direction(&key, &end, n_robots - 1));
    }

    result
}

fn moves_to_key(start: &TClusterKey, key: &TClusterKey) -> Vec<TClusterKey> {
    let mut result = Vec::new();
    let start = start.position();
    let end = key.position();

    let moving_up = start.1 > end.1;
    let moving_left = start.0 > end.0;

    if !moving_up {
        for _ in 0..start.1.abs_diff(end.1) {
            result.push(TClusterKey::Down);
        }
    }

    if moving_left {
        for _ in 0..start.0.abs_diff(end.0) {
            result.push(TClusterKey::Left);
        }
    } else {
        for _ in 0..start.0.abs_diff(end.0) {
            result.push(TClusterKey::Right);
        }
    }

    if moving_up {
        for _ in 0..start.1.abs_diff(end.1) {
            result.push(TClusterKey::Up);
        }
    }

    result.push(TClusterKey::A);
    result
}

// impl TClusterKeypadRobot1 {
//     fn new() -> Self {
//         Self {
//             position: TClusterKey::A,
//             robot: TClusterKeypadRobot2::new(),
//         }
//     }
//
//     fn enter_direction(&mut self, direction: &TClusterKey) -> String {
//         // println!("TClusterKeypadRobot1 wants {:?}", direction);
//         let mut result = String::new();
//         for key in self.moves_to_key(direction) {
//             // println!("TClusterKeypadRobot1 asks for {:?}", key);
//             result.push_str(&self.robot.enter_direction(&key));
//         }
//
//         self.position = direction.clone();
//
//         result
//     }
//
//     fn moves_to_key(&self, key: &TClusterKey) -> Vec<TClusterKey> {
//         // Always move right before up and down before left
//         let mut result = Vec::new();
//         let start = self.position.position();
//         let end = key.position();
//
//         let moving_up = start.1 > end.1;
//         let moving_left = start.0 > end.0;
//
//         if !moving_up {
//             for _ in 0..start.1.abs_diff(end.1) {
//                 result.push(TClusterKey::Down);
//             }
//         }
//
//         if moving_left {
//             for _ in 0..start.0.abs_diff(end.0) {
//                 result.push(TClusterKey::Left);
//             }
//         } else {
//             for _ in 0..start.0.abs_diff(end.0) {
//                 result.push(TClusterKey::Right);
//             }
//         }
//
//         if moving_up {
//             for _ in 0..start.1.abs_diff(end.1) {
//                 result.push(TClusterKey::Up);
//             }
//         }
//
//         result.push(TClusterKey::A);
//         result
//     }
// }
//
// impl TClusterKeypadRobot2 {
//     fn new() -> Self {
//         Self {
//             position: TClusterKey::A,
//         }
//     }
//
//     fn enter_direction(&mut self, direction: &TClusterKey) -> String {
//         // println!("TClusterKeypadRobot2 wants {:?}", direction);
//         let mut result = String::new();
//         for key in self.moves_to_key(direction) {
//             // println!("TClusterKeypadRobot2 asks for {:?}", key);
//             result.push(key.to_char());
//         }
//         self.position = direction.clone();
//
//         result
//     }
//
//     // This function will probably be wrapped in a Trait or something because it's shared
//     fn moves_to_key(&self, key: &TClusterKey) -> Vec<TClusterKey> {
//         let mut result = Vec::new();
//         let start = self.position.position();
//         let end = key.position();
//
//         let moving_up = start.1 > end.1;
//         let moving_left = start.0 > end.0;
//
//         if !moving_up {
//             for _ in 0..start.1.abs_diff(end.1) {
//                 result.push(TClusterKey::Down);
//             }
//         }
//
//         if moving_left {
//             for _ in 0..start.0.abs_diff(end.0) {
//                 result.push(TClusterKey::Left);
//             }
//         } else {
//             for _ in 0..start.0.abs_diff(end.0) {
//                 result.push(TClusterKey::Right);
//             }
//         }
//
//         if moving_up {
//             for _ in 0..start.1.abs_diff(end.1) {
//                 result.push(TClusterKey::Up);
//             }
//         }
//
//         result.push(TClusterKey::A);
//         result
//     }
// }

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
