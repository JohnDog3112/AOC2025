use std::usize;

fn part1(inp: &Vec<(usize, usize)>) -> usize {
    let mut max = 0;
    for i in 0..inp.len() {
        let (x1, y1) = inp[i];
        for j in i + 1..inp.len() {
            let (x2, y2) = inp[j];

            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            if area > max {
                max = area;
            }
        }
    }
    max
}

fn part2(inp: &Vec<(usize, usize)>) -> usize {
    let lines = inp
        .iter()
        .zip(inp.iter().skip(1).chain(inp.iter().take(1)))
        .map(|(&p1, &p2)| {
            if p1.0 < p2.0 {
                (p1, p2)
            } else if p1.1 < p2.1 {
                (p1, p2)
            } else {
                (p2, p1)
            }
        })
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..inp.len() {
        let (x1, y1) = inp[i];
        'outer: for j in i + 1..inp.len() {
            let (x2, y2) = inp[j];

            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            for &(p1, p2) in &lines {
                for ((x1_i, y1_i), (x2_i, y2_i)) in vec![
                    ((x1, y1), (x1, y2)),
                    ((x1, y1), (x2, y1)),
                    ((x2, y2), (x1, y2)),
                    ((x2, y2), (x2, y1)),
                ] {
                    if p1.0 == p2.0 && x1_i == x2_i {
                        // both horizontal
                        if !(p1.1 <= y1_i && y1_i <= p2.1 && p1.1 <= y2_i && y2_i <= p2.1)
                            && !(p1.1 == y1_i && p2.1.min(y2_i) <= p1.1 && p1.1 <= p2.1.max(y2_i))
                            && !(p2.1 == y2_i && p1.1.min(y1_i) <= p2.1 && p2.1 <= p1.1.max(y1_i))
                            && !(p1.1 == y2_i && p2.1.min(y1_i) <= p1.1 && p1.1 <= p2.1.max(y1_i))
                            && !(p2.1 == y1_i && p1.1.min(y2_i) <= p2.1 && p2.1 <= p1.1.max(y2_i))
                            && p1.0 == x1_i
                        {
                            if ((x1, y1), (x2, y2)) == ((9, 5), (2, 3))
                                || ((x2, y2), (x1, y1)) == ((9, 5), (2, 3))
                            {
                                println!("both horizontal!");
                                println!(
                                    "{:?}, {:?}, {:?}, {:?}",
                                    (x1_i, y1_i),
                                    (x2_i, y2_i),
                                    p1,
                                    p2
                                );
                            }
                            // continue 'outer;
                        }
                    } else if p1.1 == p2.1 && y1_i == y2_i {
                        // both vertical
                        if !(p1.0 <= x1_i && x1_i <= p2.0 && p1.0 <= x2_i && x2_i <= p1.0)
                            && !(p1.0 == x1_i && p2.0.min(x2_i) <= p1.0 && p1.0 <= p2.0.max(x2_i))
                            && !(p2.0 == x2_i && p1.0.min(x1_i) <= p2.0 && p2.0 <= p1.0.max(x1_i))
                            && !(p1.0 == x2_i && p2.0.min(x1_i) <= p1.0 && p1.0 <= p2.0.max(x1_i))
                            && !(p2.0 == x1_i && p1.0.min(x2_i) <= p2.0 && p2.0 <= p1.0.max(x2_i))
                            && p1.1 == y1_i
                        {
                            if ((x1, y1), (x2, y2)) == ((9, 5), (2, 3))
                                || ((x2, y2), (x1, y1)) == ((9, 5), (2, 3))
                            {
                                println!("both vertical!");
                                println!(
                                    "{:?}, {:?}, {:?}, {:?}",
                                    (x1_i, y1_i),
                                    (x2_i, y2_i),
                                    p1,
                                    p2
                                );
                            }
                            // continue 'outer;
                        }
                    } else if p1.1 == p2.1 && x1_i == x2_i {
                        // vertical and horizontal
                        if (x1_i <= p1.0 && p1.0 <= x2_i && p1.1 <= y1_i && y1_i <= p2.1) {
                            let i_x = (x1_i, p1.1);
                            if (i_x == p1 || i_x == p2)
                                && (i_x == (x1_i, y1_i) || i_x == (x2_i, y2_i))
                            {
                                println!(
                                    "({:?}, {:?}): {{{:?}, {:?}, {:?}, {:?}}} (Vertical and Horizontal)",
                                    (x1, y1),
                                    (x2, y2),
                                    (x1_i, y1_i),
                                    (x2_i, y2_i),
                                    p1,
                                    p2
                                );
                                continue;
                            }

                            if ((x1, y1), (x2, y2)) == ((9, 5), (2, 3))
                                || ((x2, y2), (x1, y1)) == ((9, 5), (2, 3))
                            {
                                println!("vertical and horizontal!");
                            }
                            continue 'outer;
                        }
                    } else if p1.0 == p2.0 && y1_i == y2_i {
                        // horizontal and vertical
                        if p1.0 <= x1_i && x1_i <= p2.0 && y1_i <= p1.1 && p1.1 <= y2_i {
                            let i_x = (p1.0, y1_i);
                            if (i_x == p1 || i_x == p2)
                                && (i_x == (x1_i, y1_i) || i_x == (x2_i, y2_i))
                            {
                                println!(
                                    "({:?}, {:?}): {{{:?}, {:?}, {:?}, {:?}}} (horizontal and vertical)",
                                    (x1, y1),
                                    (x2, y2),
                                    (x1_i, y1_i),
                                    (x2_i, y2_i),
                                    p1,
                                    p2
                                );
                                continue;
                            }

                            if ((x1, y1), (x2, y2)) == ((9, 5), (2, 3))
                                || ((x2, y2), (x1, y1)) == ((9, 5), (2, 3))
                            {
                                println!("horizontal and vertical!");
                                println!(
                                    "{:?}, {:?}, {:?}, {:?}",
                                    (x1_i, y1_i),
                                    (x2_i, y2_i),
                                    p1,
                                    p2
                                );
                            }
                            continue 'outer;
                        }
                    } else {
                        println!("{:?}, {:?}, {:?}, {:?}", (x1_i, y1_i), (x2_i, y2_i), p1, p2);
                        unreachable!()
                    }
                }
            }
            if area > max {
                println!("{area}: {:?}, {:?}", (x1, y1), (x2, y2));
                max = area;
            }
        }
    }
    max
}
fn main() {
    let inp: Vec<(usize, usize)> = include_str!("./input.txt")
        .split("\n")
        .map(|line| {
            let parts: Vec<_> = line.split(",").map(|num| num.parse().unwrap()).collect();
            assert!(parts.len() == 2);
            (parts[0], parts[1])
        })
        .collect();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
