use std::cmp::max;
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day09.txt")?;
    let reader = BufReader::new(f);
    let input = reader.lines().nth(0).unwrap()?;

    let part1_start = std::time::Instant::now();
    let part1_result = part1(&input);
    let part2_start = std::time::Instant::now();
    let part2_result = part2(&input);
    let end = std::time::Instant::now();

    println!(
        "Part1: {}, duration: {:?}",
        part1_result,
        part2_start.duration_since(part1_start)
    );
    println!(
        "Part2: {}, duration: {:?}",
        part2_result,
        end.duration_since(part2_start)
    );

    Ok(())
}

fn part1(input: &str) -> usize {
    let mut result = Vec::new();
    let mut stack = LinkedList::new();
    let mut to_fill = LinkedList::new();
    let mut index = 0usize;
    let mut file_num = 0usize;

    for (i, c) in input.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                result.push(Some(file_num));
                stack.push_front((file_num, index));
            } else {
                result.push(None);
                to_fill.push_back(index);
            }
            index += 1;
        }
        if i % 2 == 0 {
            file_num += 1;
        }
    }

    for i in to_fill.into_iter() {
        if is_compact(&result) {
            break;
        }
        let (val, index) = stack.pop_front().unwrap();
        result.remove(index);
        result[i] = Some(val);
    }

    checksum(&result)
}

#[derive(Debug)]
struct Block {
    id: usize,
    start: usize,
    len: usize,
}

impl Block {
    pub(crate) fn clone(&self) -> Self {
        Self {
            id: self.id,
            start: self.start,
            len: self.len,
        }
    }
}

fn part2(input: &str) -> usize {
    let mut stack = LinkedList::new();
    let mut free_list = LinkedList::new(); // this is faster as a Vec!

    let mut index = 0usize;
    let mut max_index = 0usize;
    let mut file_id = 0usize;

    for (i, c) in input.chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            stack.push_front(Block {
                id: file_id,
                start: index,
                len,
            });
            file_id += 1;
        } else {
            free_list.push_back(Block {
                id: 0,
                start: index,
                len,
            });
        }
        index += len;
        max_index = max(index, max_index);
    }

    let mut blocks = LinkedList::new();

    for block in stack.iter() {
        let mut moved = false;
        for free_block in free_list.iter_mut() {
            // Make sure not to move any blocks to the right!
            if free_block.start < block.start && free_block.len >= block.len {
                blocks.push_back(Block {
                    id: block.id,
                    start: free_block.start,
                    len: block.len,
                });
                free_block.len -= block.len;
                free_block.start += block.len;
                moved = true;
                break;
            }
        }
        if !moved {
            blocks.push_back(block.clone());
        }
    }

    let mut result = vec![None; max_index + 1];
    for block in blocks.iter() {
        for i in block.start..(block.start + block.len) {
            result[i] = Some(block.id);
        }
    }

    checksum(&result[..])
}

fn checksum(v: &[Option<usize>]) -> usize {
    let mut checksum = 0;
    for (i, n) in v.iter().enumerate() {
        match n {
            Some(v) => checksum += i * v,
            None => {}
        }
    }
    checksum
}

fn is_compact(v: &Vec<Option<usize>>) -> bool {
    let total = v.iter().filter(|v| v.is_some()).count();

    v[0..total].iter().all(|v| v.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let f = File::open("../test_input/day09test.txt")?;
        let reader = BufReader::new(f);
        let input = reader.lines().nth(0).unwrap()?;
        assert_eq!(part1(&input), 1928);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let f = File::open("../test_input/day09test.txt")?;
        let reader = BufReader::new(f);
        let input = reader.lines().nth(0).unwrap()?;
        assert_eq!(part2(&input), 2858);

        Ok(())
    }
}
