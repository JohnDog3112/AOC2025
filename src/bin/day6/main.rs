#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Times,
}
struct Operation {
    nums: Vec<String>,
    operator: Operator,
}

fn part1(inp: &Vec<Operation>) -> usize {
    inp.into_iter()
        .map(|operation| {
            operation
                .nums
                .iter()
                .map(|num| num.trim().parse::<usize>().unwrap())
                .reduce(|acc, next| match operation.operator {
                    Operator::Plus => acc + next,
                    Operator::Times => acc * next,
                })
                .unwrap()
        })
        .sum()
}

fn part2(inp: &Vec<Operation>) -> usize {
    inp.into_iter()
        .map(|operation| {
            operation
                .nums
                .iter()
                .fold(
                    vec!["".to_string(); operation.nums[0].len()],
                    |mut acc, next| {
                        for (i, ch) in next.chars().enumerate() {
                            match ch {
                                ' ' => (),
                                a => acc[i] += a.to_string().as_ref(),
                            }
                        }
                        acc
                    },
                )
                .into_iter()
                .map(|num| num.parse::<usize>().unwrap())
                .reduce(|acc, next| match operation.operator {
                    Operator::Plus => acc + next,
                    Operator::Times => acc * next,
                })
                .unwrap()
        })
        .sum()
}
fn main() {
    let inp = include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>();
    let mut offsets = vec![];
    let mut operators = vec![];
    for (i, ch) in inp[inp.len() - 1].chars().enumerate() {
        match ch {
            '*' => {
                operators.push(Operator::Times);
                offsets.push(i);
            }
            '+' => {
                operators.push(Operator::Plus);
                offsets.push(i);
            }
            ' ' => (),
            _ => unreachable!(),
        }
    }
    offsets.push(inp[inp.len() - 1].len() + 1);

    let nums =
        inp[0..inp.len() - 1]
            .into_iter()
            .fold(vec![vec![]; operators.len()], |acc, next| {
                offsets
                    .clone()
                    .into_iter()
                    .zip(offsets.clone().into_iter().skip(1))
                    .map(|(min, max)| next[min..max - 1].to_string())
                    .zip(acc)
                    .map(|(next, mut acc)| {
                        acc.push(next);
                        acc
                    })
                    .collect()
            });

    let inp = nums
        .into_iter()
        .zip(operators.into_iter())
        .map(|(nums, operator)| Operation { nums, operator })
        .collect();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
