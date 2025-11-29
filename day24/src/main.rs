use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let mut circuit = DigitalCircuit::from_file("input/day24.txt")?;

    circuit.evaluate_all();

    let mut z_wires = circuit.wires.keys()
        .filter(|s| s.starts_with('z'))
        .collect::<Vec<_>>();
    z_wires.sort();
    let z_wires: Vec<bool> = z_wires.into_iter()
        .map(|wire| {
            match circuit.wires[wire] {
                Value::Literal(value) => value,
                _ => panic!("Unevaluated wire!"),
            }
        })
        .collect();
    let part1 = z_wires.iter()
        .rev()
        .fold(0, |acc, &b| (acc << 1) | b as u64);
    println!("Part 1: {part1}");

    Ok(())
}

#[derive(Debug, Clone)]
enum Value {
    Literal(bool),
    Expr(String, Gate, String),
}

#[derive(Debug, Clone)]
enum Gate {
    AND,
    OR,
    XOR,
}

impl Gate {
    fn from_str(s: &str) -> Option<Gate> {
        match s {
            "AND" => Some(Gate::AND),
            "OR" => Some(Gate::OR),
            "XOR" => Some(Gate::XOR),
            _ => None,
        }
    }

    fn evaluate(&self, left: bool, right: bool) -> bool {
        match self {
            Gate::AND => left && right,
            Gate::OR => left || right,
            Gate::XOR => left ^ right,
        }
    }
}

#[derive(Debug, Clone)]
struct DigitalCircuit {
    wires: HashMap<String, Value>,
}

impl DigitalCircuit {
    fn from_file(file_name: impl AsRef<Path>) -> anyhow::Result<Self> {
        let f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        let mut wires = HashMap::new();

        // Read in initial wire values
        while reader.read_line(&mut buffer)? > 0 {
            let line = buffer.trim();
            if line.is_empty() {
                break;
            }
            let tokens = line.split(':').map(|s| s.trim()).collect::<Vec<&str>>();
            let wire = tokens[0];
            let value = match tokens[1] {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
            wires.insert(String::from(wire), Value::Literal(value));
            buffer.clear();
        }

        // Read in circuit structure
        while reader.read_line(&mut buffer)? > 0 {
            let line = buffer.trim();
            if line.is_empty() {
                break;
            }
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let input0 = tokens[0];
            let gate = Gate::from_str(&tokens[1]).unwrap();
            let input1 = tokens[2];
            let output_wire = tokens[4];

            wires.insert(
                String::from(output_wire),
                Value::Expr(String::from(input0), gate, String::from(input1)),
            );
            buffer.clear();
        }

        Ok(Self { wires })
    }

    fn evaluate_all(&mut self) {
        let all_keys = self.wires.keys().cloned().collect::<HashSet<_>>();
        for wire in &all_keys {
            let _ = evaluate(wire.as_str(), &mut self.wires);
        }
    }
}

fn evaluate(wire: &str, map: &mut HashMap<String, Value>) -> anyhow::Result<bool> {
    if let Some(Value::Literal(b)) = map.get(wire) {
        return Ok(*b);
    }
    let (input0, gate, input1) = match map.get(wire) {
        Some(Value::Expr(input0, gate, input1)) => (input0.clone(), gate.clone(), input1.clone()),
        _ => bail!("Unknown wire {}", wire),
    };
    let input0 = evaluate(&input0, map)?;
    let input1 = evaluate(&input1, map)?;
    let result = gate.evaluate(input0, input1);
    if let Some(val) = map.get_mut(wire) {
        *val = Value::Literal(result);
    }

    Ok(result)
}
