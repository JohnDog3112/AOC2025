use std::collections::HashMap;

fn part1_helper(
    next: String,
    inp: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(val) = cache.get(&next) {
        return *val;
    }
    if next == "out" {
        return 1;
    }

    let res = inp
        .get(&next)
        .unwrap()
        .into_iter()
        .map(|connection| part1_helper(connection.clone(), inp, cache))
        .sum();

    assert!(cache.insert(next, res).is_none());

    res
}
fn part1(inp: &HashMap<String, Vec<String>>) -> usize {
    part1_helper("you".to_string(), inp, &mut HashMap::new())
}

fn part2_helper(
    next: String,
    inp: &HashMap<String, Vec<String>>,
    visited: &mut [bool; 2],
    cache: &mut HashMap<(String, [bool; 2]), usize>,
) -> usize {
    if let Some(val) = cache.get(&(next.clone(), visited.clone())) {
        return *val;
    }
    if next == "out" {
        if visited[0] && visited[1] {
            return 1;
        } else {
            return 0;
        }
    }

    if next == "dac" {
        assert!(!visited[0]);
        visited[0] = true;
    } else if next == "fft" {
        assert!(!visited[1]);
        visited[1] = true;
    }

    let res = inp
        .get(&next)
        .unwrap()
        .into_iter()
        .map(|connection| part2_helper(connection.clone(), inp, visited, cache))
        .sum();

    if let Some(val) = cache.get(&(next.clone(), visited.clone())) {
        assert!(*val == res);
    } else {
        cache.insert((next.clone(), visited.clone()), res);
    }

    if next == "dac" {
        assert!(visited[0]);
        visited[0] = false;
    } else if next == "fft" {
        assert!(visited[1]);
        visited[1] = false;
    }

    res
}
fn part2(inp: &HashMap<String, Vec<String>>) -> usize {
    part2_helper("svr".to_string(), inp, &mut [false; 2], &mut HashMap::new())
}
fn main() {
    let inp = include_str!("./input.txt")
        .split("\n")
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            let key = &parts[0][0..parts[0].len() - 1];
            let connections = parts[1..]
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>();

            (key.to_string(), connections)
        })
        .collect::<HashMap<String, Vec<String>>>();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
