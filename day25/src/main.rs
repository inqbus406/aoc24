use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let f = File::open("input/day25.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut keys = HashSet::new();
    let mut locks = HashSet::new();

    let mut buffer = String::new();
    let mut is_lock = false;
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            // create key or lock from buffer
            if is_lock {
                locks.insert(Lock::from_str(&buffer)?);
            } else {
                keys.insert(Key::from_str(&buffer)?);
            }
            buffer.clear();
            is_lock = false;
            continue;
        }
        if buffer.is_empty() && line == "#####" {
            is_lock = true;
        }
        buffer.push_str(&line);
        buffer.push('\n');
    }
    if is_lock {
        locks.insert(Lock::from_str(&buffer)?);
    } else {
        keys.insert(Key::from_str(&buffer)?);
    }

    let part1 = locks.iter().fold(0, |acc, lock| {
        acc + keys.iter().filter(|&key| lock.accepts(key)).count()
    });
    println!("Part 1: {}", part1);

    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Key {
    heights: Vec<u8>,
}

impl Key {
    fn from_str(s: &str) -> anyhow::Result<Key> {
        let mut v = Vec::new();
        for line in s.lines() {
            v.push(line.chars().collect::<Vec<char>>());
        }

        let mut heights = Vec::new();

        for i in 0..v[0].len() {
            let mut height = 0;
            for j in (0..(v.len() - 1)).rev() {
                if v[j][i] != '#' {
                    break;
                }
                height += 1;
            }
            heights.push(height);
        }

        Ok(Self { heights })
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Lock {
    heights: Vec<u8>,
}

impl Lock {
    fn from_str(s: &str) -> anyhow::Result<Lock> {
        let mut v = Vec::new();
        for line in s.lines() {
            v.push(line.chars().collect::<Vec<char>>());
        }

        let mut heights = Vec::new();

        for i in 0..v[0].len() {
            let mut height = 0;
            for j in 1..v.len() {
                if v[j][i] != '#' {
                    break;
                }
                height += 1;
            }
            heights.push(height);
        }

        Ok(Self { heights })
    }

    fn accepts(&self, key: &Key) -> bool {
        // println!("Checking {:?} against {:?}", key, &self);
        for (x, y) in self.heights.iter().zip(key.heights.clone()) {
            if x + y > 5 {
                // println!("Doesn't fit");
                return false;
            }
        }

        // println!("fits!");
        true
    }
}
