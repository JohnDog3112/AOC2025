fn part1(inp: &Vec<Vec<u32>>) -> u32 {
    inp.into_iter()
        .map(|bank| {
            let (num1, num2) =
                bank.into_iter()
                    .enumerate()
                    .fold((0, 0), |(num1, num2), (i, &next)| {
                        if next > num1 && i != bank.len() - 1 {
                            (next, 0)
                        } else if next > num2 {
                            (num1, next)
                        } else {
                            (num1, num2)
                        }
                    });
            num1 * 10 + num2
        })
        .sum()
}

fn part2(inp: &Vec<Vec<u32>>) -> u128 {
    inp.into_iter()
        .map(|bank| {
            bank.into_iter()
                .enumerate()
                .fold([0; 12], |mut nums, (i, &next)| {
                    for (j, &num) in nums.iter().enumerate() {
                        if num < next && 12 - j <= bank.len() - i {
                            nums[j + 1..].fill(0);
                            nums[j] = next;
                            return nums;
                        }
                    }
                    nums
                })
                .into_iter()
                .fold(0, |acc, next| acc * 10 + (next as u128))
        })
        .sum()
}

fn main() {
    let inp = include_str!("./input.txt")
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
