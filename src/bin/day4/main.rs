#[derive(Clone, Copy, PartialEq, Eq)]
enum MapPiece {
    Paper,
    Empty,
}

fn get_offset(
    map: &Vec<Vec<MapPiece>>,
    x: usize,
    y: usize,
    x_offset: i32,
    y_offset: i32,
) -> Option<MapPiece> {
    if x_offset < 0 && x_offset.abs() > x as i32 {
        None
    } else if y_offset < 0 && y_offset.abs() > y as i32 {
        None
    } else if (x as i32 + x_offset) as usize >= map.len() {
        None
    } else if (y as i32 + y_offset) as usize >= map[(x as i32 + x_offset) as usize].len() {
        None
    } else {
        Some(map[(x as i32 + x_offset) as usize][(y as i32 + y_offset) as usize])
    }
}

fn part1(inp: &Vec<Vec<MapPiece>>) -> usize {
    let offsets = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .filter(|(i, j)| !(i == j && *i == 0))
        .collect::<Vec<(i32, i32)>>();

    inp.into_iter()
        .enumerate()
        .map(|(i, row)| {
            let offsets = offsets.clone();
            row.into_iter().enumerate().filter(move |(j, loc)| {
                **loc == MapPiece::Paper
                    && offsets
                        .clone()
                        .into_iter()
                        .filter(|(i_o, j_o)| {
                            get_offset(&inp, i, *j, *i_o, *j_o) == Some(MapPiece::Paper)
                        })
                        .count()
                        < 4
            })
        })
        .flatten()
        .count()
}

fn part2(inp: &Vec<Vec<MapPiece>>) -> usize {
    let offsets = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .filter(|(i, j)| !(i == j && *i == 0))
        .collect::<Vec<(i32, i32)>>();

    let mut prev_count = 1;
    let mut count = 0;
    let mut map = inp.clone();

    while prev_count != count {
        prev_count = count;

        map = map
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let offsets = offsets.clone();
                row.into_iter()
                    .enumerate()
                    .map(|(j, loc)| {
                        let remove = loc == MapPiece::Paper
                            && offsets
                                .clone()
                                .into_iter()
                                .filter(|(i_o, j_o)| {
                                    get_offset(&map, i, j, *i_o, *j_o) == Some(MapPiece::Paper)
                                })
                                .count()
                                < 4;

                        if remove {
                            count += 1;
                            MapPiece::Empty
                        } else {
                            loc
                        }
                    })
                    .collect()
            })
            .collect()
    }
    count
}

fn main() {
    let inp = include_str!("./input.txt")
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => MapPiece::Empty,
                    '@' => MapPiece::Paper,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<MapPiece>>>();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
