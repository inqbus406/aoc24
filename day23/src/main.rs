use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let dir = if args.contains(&String::from("--test")) {
        PathBuf::from("test_input")
    } else {
        PathBuf::from("input")
    };

    let f = File::open(dir.join("day23.txt"))?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut connections = Vec::new();
    for line in lines {
        let Ok(connection) = line else {
            continue;
        };

        let nodes = connection.split('-').map(|s| s.trim()).collect::<Vec<_>>();
        connections.push((nodes[0].to_string(), nodes[1].to_string()));
    }

    let part1 = part1(&connections);

    println!("Part 1: {}", part1);

    Ok(())
}

fn part1(connections: &Vec<(String, String)>) -> usize {
    fn is_triangle(connection1: &(String, String), connection2: &(String, String)) -> bool {
        connection1.0 == connection2.0
            || connection1.0 == connection2.1
            || connection1.1 == connection2.0
            || connection1.1 == connection2.1
    }

    let mut result = 0;

    for (i, node) in connections.iter().enumerate() {
    }

    for i in 0..(connections.len() - 1) {
        'inner: for j in (i + 1)..connections.len() {
            let nodes = [
                connections[i].0.as_str(),
                connections[j].0.as_str(),
                connections[i].1.as_str(),
                connections[j].1.as_str(),
            ];
            if !nodes.iter().any(|n| n.starts_with('t')) {
                continue 'inner;
            }
            if is_triangle(&connections[i], &connections[j]) {
                let mut triangle = HashSet::new();
                triangle.insert(connections[j].0.clone());
                triangle.insert(connections[j].1.clone());
                triangle.insert(connections[i].0.clone());
                triangle.insert(connections[i].1.clone());
                println!("Triangle: {:?}", &triangle);
                result += 1;
            }
        }
    }

    result
}
