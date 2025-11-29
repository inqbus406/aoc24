use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Crops = HashMap<char, Vec<HashSet<Point>>>;

fn main() -> std::io::Result<()> {
    let map = Map::from_file("input/day12.txt")?;

    let mut crops = Crops::new();
    let mut explored = HashSet::new();
    let mut current_point = Point { x: 0, y: 0 };
    let mut fringe = VecDeque::new();

    while explored.len() < map.size() {
        if explored.contains(&current_point) {
            current_point = fringe.pop_front().unwrap();
            continue;
        }

        let c = map.get(&current_point);
        if !crops.contains_key(&c) {
            let mut set = HashSet::new();
            set.insert(current_point);
            crops.insert(c, vec![set]);
        } else {
            let mut found = false;
            for group in crops.get_mut(&c).unwrap() {
                if group.iter().any(|point| point.adjacent(&current_point)) {
                    group.insert(current_point);
                    found = true;
                }
            }
            if !found {
                let mut set = HashSet::new();
                set.insert(current_point);
                crops.get_mut(&c).unwrap().push(set);
            }
        }

        explored.insert(current_point);
        for neighbor in map.get_neighbors(&current_point) {
            if !explored.contains(&neighbor) {
                fringe.push_back(neighbor);
            }
        }
        current_point = fringe.pop_front().unwrap();
    }

    for (_, regions) in &mut crops {
        // println!("Checking {}", c);
        // dbg!(&regions.len());
        loop {
            let new_regions = combine_sets(regions);
            if new_regions.len() == regions.len() {
                break;
            }
            regions.clear();
            regions.extend(new_regions);
        }
    }

    println!("Part 1: {}", part1(&crops));
    println!("Part 2: {}", part2(&crops, &map));

    Ok(())
}

fn part1(crops: &Crops) -> usize {
    let mut result = 0;
    for regions in crops.values() {
        for region in regions {
            let area = area(region);
            let perimeter = perimeter(region);
            let product = area * perimeter;
            result += product;
        }
    }

    result
}

fn part2(crops: &Crops, map: &Map) -> usize {
    let mut result = 0;

    for regions in crops.values() {
        for region in regions {
            let area = area(region);
            let corners = region
                .iter()
                .map(|p| corners(p, region, &map))
                .sum::<usize>();
            let product = area * corners;
            result += product;
        }
    }

    result
}

fn corners(p: &Point, region: &HashSet<Point>, map: &Map) -> usize {
    if !region.contains(p) {
        panic!();
    }
    let all_neighbors = map.get_neighbors_nodiag(p);
    let all_neighbors_diag = map.get_neighbors_diag(p);

    let neighbors = all_neighbors
        .iter()
        .filter(|neighbor| region.contains(neighbor))
        .collect_vec();
    let neighbors_diag = all_neighbors_diag
        .iter()
        .filter(|neighbor| region.contains(neighbor))
        .collect_vec();

    let result = match neighbors.len() {
        0 => 4,
        1 => 2,
        2 => {
            if neighbors.iter().all(|neighbor| neighbor.x == p.x)
                || neighbors.iter().all(|neighbor| neighbor.y == p.y)
            {
                // middle of a straight piece, no corners
                0
            } else {
                // center of an L
                if region.contains(&Point {
                    x: neighbors[0].x,
                    y: neighbors[1].y,
                }) && region.contains(&Point {
                    x: neighbors[1].x,
                    y: neighbors[0].y,
                }) {
                    1
                } else {
                    2
                }
            }
        }
        3 => {
            // T with or without diagonal neighbors
            let mut result = 2usize;
            let same_x = neighbors
                .iter()
                .filter(|neighbor| neighbor.x == p.x)
                .collect_vec();
            let same_y = neighbors
                .iter()
                .filter(|neighbor| neighbor.y == p.y)
                .collect_vec();
            if same_x.len() == 2 {
                // sideways T or reverse
                if neighbors_diag.contains(&&Point {
                    x: same_y[0].x,
                    y: same_x[0].y,
                }) {
                    result -= 1;
                }
                if neighbors_diag.contains(&&Point {
                    x: same_y[0].x,
                    y: same_x[1].y,
                }) {
                    result -= 1;
                }
            } else {
                // proper T or upside down
                if neighbors_diag.contains(&&Point {
                    x: same_y[0].x,
                    y: same_x[0].y,
                }) {
                    result -= 1;
                }
                if neighbors_diag.contains(&&Point {
                    x: same_y[1].x,
                    y: same_x[0].y,
                }) {
                    result -= 1;
                }
            }
            result
        }
        4 => 4 - neighbors_diag.len(),
        _ => unreachable!(),
    };

    // println!("{:?} has {} corners!", p, result);
    result
}

fn combine_sets(sets: &Vec<HashSet<Point>>) -> Vec<HashSet<Point>> {
    let mut regions_coalesced = Vec::new();
    for (i, region0) in sets.iter().enumerate() {
        for j in (i + 1)..sets.len() {
            let region1 = &sets[j];
            if region0.intersection(region1).count() > 0 {
                let combined: HashSet<Point> = region0.union(region1).cloned().collect();
                regions_coalesced.push(combined);
                for (k, region) in sets.iter().enumerate() {
                    if k != j && k != i {
                        regions_coalesced.push(region.clone());
                    }
                }
                return regions_coalesced;
            }
        }
    }
    sets.clone()
}

fn area(region: &HashSet<Point>) -> usize {
    region.len()
}

fn perimeter(region: &HashSet<Point>) -> usize {
    4 * region.len()
        - (2 * region
            .iter()
            .combinations(2)
            .filter(|x| x[0].adjacent(x[1]))
            .count())
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();
        let mut input = Vec::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            input.push(line.chars().collect_vec());
        }

        Ok(Self { map: input })
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x < self.map[0].len() as i32
            && point.y >= 0
            && point.y < self.map.len() as i32
    }

    fn get_neighbors(&self, start: &Point) -> Vec<Point> {
        vec![
            Point {
                x: start.x + 1,
                y: start.y,
            },
            Point {
                x: start.x - 1,
                y: start.y,
            },
            Point {
                x: start.x,
                y: start.y - 1,
            },
            Point {
                x: start.x,
                y: start.y + 1,
            },
            Point {
                x: start.x + 1,
                y: start.y + 1,
            },
        ]
        .into_iter()
        .filter(|p| self.in_bounds(p))
        .collect()
    }

    fn get_neighbors_nodiag(&self, start: &Point) -> Vec<Point> {
        vec![
            Point {
                x: start.x + 1,
                y: start.y,
            },
            Point {
                x: start.x - 1,
                y: start.y,
            },
            Point {
                x: start.x,
                y: start.y - 1,
            },
            Point {
                x: start.x,
                y: start.y + 1,
            },
        ]
        .into_iter()
        .filter(|p| self.in_bounds(p))
        .collect()
    }

    fn get_neighbors_diag(&self, start: &Point) -> Vec<Point> {
        vec![
            Point {
                x: start.x + 1,
                y: start.y + 1,
            },
            Point {
                x: start.x + 1,
                y: start.y - 1,
            },
            Point {
                x: start.x - 1,
                y: start.y + 1,
            },
            Point {
                x: start.x - 1,
                y: start.y - 1,
            },
        ]
        .into_iter()
        .filter(|p| self.in_bounds(p))
        .collect()
    }

    fn size(&self) -> usize {
        self.map[0].len() * self.map.len()
    }

    fn get(&self, point: &Point) -> char {
        if !self.in_bounds(point) {
            panic!();
        }
        self.map[point.y as usize][point.x as usize]
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn adjacent(&self, other: &Point) -> bool {
        if other.y == self.y {
            if (other.x - self.x).abs() == 1 {
                return true;
            }
            return false;
        }
        if other.x == self.x {
            if (other.y - self.y).abs() == 1 {
                return true;
            }
            return false;
        }

        false
        // match other {
        //     Point {x, y } if *x == self.x + 1 && *y == self.y => true,
        //     Point {x, y } if *x == self.x - 1 && *y == self.y => true,
        //     Point {x, y } if *x == self.x && *y == self.y + 1 => true,
        //     Point {x, y } if *x == self.x && *y == self.y - 1 => true,
        //     _ => false
        // }
    }
}
