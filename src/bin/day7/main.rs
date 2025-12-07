use std::collections::{HashMap, HashSet};

struct Input {
    splitters: HashMap<usize, Vec<usize>>,
    start: (usize, usize),
}

fn part1(inp: &Input) -> usize {
    let mut beams = vec![inp.start];

    let mut splitters_used: HashSet<(usize, usize)> = HashSet::new();
    while let Some((beam_x, beam_y)) = beams.pop() {
        if let Some(splitters) = inp.splitters.get(&beam_x) {
            for &splitter_y in splitters {
                if splitter_y >= beam_y {
                    if !splitters_used.contains(&(beam_x, splitter_y)) {
                        splitters_used.insert((beam_x, splitter_y));
                        beams.push((beam_x + 1, splitter_y));
                        if beam_x != 0 {
                            beams.push((beam_x - 1, splitter_y));
                        }
                    }
                    break;
                }
            }
        }
    }

    splitters_used.len()
}

fn part2_helper(
    (beam_x, beam_y): (usize, usize),
    inp: &Input,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(splitters) = inp.splitters.get(&beam_x) {
        for &splitter_y in splitters {
            if splitter_y >= beam_y {
                if let Some(val) = cache.get(&(beam_x, splitter_y)) {
                    return *val;
                }
                let branch1 = part2_helper((beam_x + 1, splitter_y), inp, cache);
                let branch2 = if beam_x > 0 {
                    part2_helper((beam_x - 1, splitter_y), inp, cache)
                } else {
                    1
                };

                cache.insert((beam_x, splitter_y), branch1 + branch2);
                return branch1 + branch2;
            }
        }
    }
    1
}

fn part2(inp: &Input) -> usize {
    part2_helper(inp.start, inp, &mut HashMap::new())
}
fn main() {
    let mut start = (0, 0);
    let mut inp: HashMap<usize, Vec<usize>> = include_str!("./input.txt")
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| match ch {
                    'S' => {
                        start = (x, y);
                        None
                    }
                    '.' => None,
                    '^' => Some((x, y)),
                    _ => unreachable!(),
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .fold(HashMap::new(), |mut acc, next| {
            for (x, y) in next {
                if let Some(vals) = acc.get_mut(&x) {
                    vals.push(y);
                } else {
                    acc.insert(x, vec![y]);
                }
            }
            acc
        });
    for (_, vals) in &mut inp {
        vals.sort();
    }

    let inp = Input {
        splitters: inp,
        start,
    };

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
