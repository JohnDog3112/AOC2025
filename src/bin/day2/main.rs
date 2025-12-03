struct Range {
    min: u64,
    max: u64,
}

fn part1(inp: &Vec<Range>) -> u64 {
    let mut sum = 0;
    for range in inp {
        for num in range.min..=range.max {
            let st = num.to_string();
            if st.len() % 2 == 1 {
                continue;
            }
            let st = st.chars().collect::<Vec<_>>();
            if st[0..st.len() / 2] == st[st.len() / 2..] {
                sum += num;
            }
        }
    }

    sum
}

fn part2(inp: &Vec<Range>) -> u64 {
    let mut sum = 0;
    for range in inp {
        'outer: for num in range.min..=range.max {
            let st = num.to_string();
            let st = st.chars().collect::<Vec<_>>();

            'inner: for len in 1..=st.len() / 2 {
                if st.len() % len != 0 {
                    continue;
                }
                for j in 1..st.len() / len {
                    let offset = j * len;
                    if st[0..len] != st[offset..offset + len] {
                        continue 'inner;
                    }
                }
                sum += num;
                continue 'outer;
            }
        }
    }

    sum
}

fn main() {
    let inp: Vec<Range> = include_str!("./input.txt")
        .split(",")
        .map(|part| {
            let parts: Vec<_> = part.split("-").collect();
            assert!(parts.len() == 2);

            Range {
                min: parts[0].parse().unwrap(),
                max: parts[1].parse().unwrap(),
            }
        })
        .collect();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
