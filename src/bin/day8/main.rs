use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Connection {
    start: (i128, i128, i128),
    end: (i128, i128, i128),
    squared_distance: i128,
}
impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.squared_distance.partial_cmp(&other.squared_distance)
    }
}
impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.squared_distance.cmp(&other.squared_distance)
    }
}

impl Connection {
    fn get_circuits(&self, circuits: &Vec<HashSet<(i128, i128, i128)>>) -> (usize, usize) {
        let mut start_group = None;
        for i in 0..circuits.len() {
            if circuits[i].contains(&self.start) {
                start_group = Some(i);
                break;
            }
        }
        let mut end_group = None;
        for i in 0..circuits.len() {
            if circuits[i].contains(&self.end) {
                end_group = Some(i);
                break;
            }
        }

        let start_group = start_group.unwrap();
        let end_group = end_group.unwrap();
        if start_group < end_group {
            (start_group, end_group)
        } else {
            (end_group, start_group)
        }
    }
}

fn part1(
    mut connections: BinaryHeap<Reverse<Connection>>,
    mut circuits: Vec<HashSet<(i128, i128, i128)>>,
) -> usize {
    for _ in 0..999 {
        let next = connections.pop().unwrap().0;

        let (start_group, end_group) = next.get_circuits(&circuits);

        if start_group == end_group {
            continue;
        }

        let group = circuits.remove(end_group);
        circuits[start_group].extend(group);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    circuits
        .into_iter()
        .take(3)
        .map(|item| item.len())
        .reduce(|acc, next| acc * next)
        .unwrap()
}

fn part2(
    mut connections: BinaryHeap<Reverse<Connection>>,
    mut circuits: Vec<HashSet<(i128, i128, i128)>>,
) -> usize {
    loop {
        let next = connections.pop().unwrap().0;

        let (start_group, end_group) = next.get_circuits(&circuits);

        if start_group == end_group {
            continue;
        }

        let group = circuits.remove(end_group);
        circuits[start_group].extend(group);

        if circuits.len() == 1 {
            return (next.start.0 * next.end.0) as usize;
        }
    }
}

fn main() {
    let inp = include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let cords: Vec<i128> = line.split(",").map(|cord| cord.parse().unwrap()).collect();
            assert!(cords.len() == 3);
            (cords[0], cords[1], cords[2])
        })
        .collect::<Vec<_>>();

    let mut connections: BinaryHeap<Reverse<Connection>> = BinaryHeap::new();

    for i in 0..inp.len() {
        let start = inp[i];
        for j in i + 1..inp.len() {
            let end = inp[j];
            connections.push(Reverse(Connection {
                start,
                end,
                squared_distance: (start.0 - end.0).abs().pow(2)
                    + (start.1 - end.1).abs().pow(2)
                    + (start.2 - end.2).abs().pow(2),
            }));
        }
    }

    let circuits: Vec<HashSet<(i128, i128, i128)>> = inp
        .clone()
        .into_iter()
        .map(|junction| {
            let mut hash = HashSet::new();
            hash.insert(junction);
            hash
        })
        .collect();

    println!("part1: {}", part1(connections.clone(), circuits.clone()));
    println!("part2: {}", part2(connections, circuits));
}
