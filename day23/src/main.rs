use graph_cycles::Cycles;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("test_input/day23.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut g = UnGraph::<String, ()>::new_undirected();
    let mut indices = HashMap::new();
    for line in lines {
        let Ok(connection) = line else {
            continue;
        };

        let nodes = connection.split('-').map(|s| s.trim()).collect::<Vec<_>>();
        let node0 = indices
            .entry(nodes[0].to_string())
            .or_insert_with(|| g.add_node(nodes[0].to_string()))
            .clone();
        let node1 = indices
            .entry(nodes[1].to_string())
            .or_insert_with(|| g.add_node(nodes[1].to_string()))
            .clone();
        g.add_edge(node0, node1, ());
    }

    let triangles = g
        .cycles()
        .into_iter()
        .filter(|c| c.len() == 3)
        .map(|v| Triangle::from_vec(&v, &indices))
        .filter(|t| t.one_starts_with_t())
        .collect::<HashSet<Triangle>>();
    // dbg!(&triangles)
    println!("Part 1: {}", triangles.len());

    // let basic_dot = Dot::new(&g);
    // println!("Basic DOT format:\n{:?}\n", basic_dot);

    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
struct Triangle {
    nodes: BTreeSet<String>,
}

impl Triangle {
    fn from_vec(input: &Vec<NodeIndex>, indices: &HashMap<String, NodeIndex>) -> Self {
        let mut nodes = BTreeSet::new();
        for node in input {
            for (key, value) in indices {
                if value == node {
                    nodes.insert(key.to_string());
                }
            }
        }

        Self { nodes }
    }

    fn one_starts_with_t(&self) -> bool {
        self.nodes.iter().any(|s| s.starts_with('t'))
    }
}

impl Hash for Triangle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for node in &self.nodes {
            node.hash(state);
        }
    }
}
