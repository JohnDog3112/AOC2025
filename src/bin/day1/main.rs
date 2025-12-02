#[derive(Debug)]
enum Input {
    Left(u32),
    Right(u32),
}

fn part1(input: &Vec<Input>) -> i32 {
    let (count, _) = input.iter().fold((0, 50), |(count, pos), current_val| {
        let change = match current_val {
            Input::Left(val) => -(*val as i64),
            Input::Right(val) => *val as i64,
        };

        let pos = (pos + 100 - (change % 100)) % 100;
        if pos == 0 {
            (count + 1, pos)
        } else {
            (count, pos)
        }
    });

    count
}

fn part2(input: &Vec<Input>) -> u32 {
    let (count, _) = input
        .iter()
        .fold((0, 50), |(mut count, mut pos), current_val| {
            match current_val {
                Input::Left(val) => {
                    count += val / 100;
                    let val = val % 100;

                    if val >= pos {
                        if pos != 0 {
                            count += 1;
                        }
                        pos = (pos + 100 - val) % 100;
                    } else {
                        pos -= val;
                    }
                }
                Input::Right(val) => {
                    count += val / 100;
                    let val = val % 100;

                    pos += val;
                    if pos >= 100 {
                        count += 1;
                        pos -= 100;
                    }
                }
            }

            (count, pos)
        });

    count
}

fn main() {
    let input = include_str!("./input.txt");
    let input: Vec<_> = input
        .split("\n")
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let num: u32 = chars[1..].into_iter().collect::<String>().parse().unwrap();
            match chars[0] {
                'R' => Input::Right(num),
                'L' => Input::Left(num),
                a => unreachable!("Unknown starting character: {a}"),
            }
        })
        .collect();

    println!("part1: {}", part1(&input));

    println!("part2: {}", part2(&input));
}
