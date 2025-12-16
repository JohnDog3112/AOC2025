use std::collections::HashSet;

#[derive(Clone)]
struct Present(Vec<Vec<bool>>);

impl Present {
    fn flip_y(self) -> Self {
        Present(
            self.0
                .into_iter()
                .map(|mut row| {
                    let a = row[0];
                    row[0] = row[2];
                    row[2] = a;
                    row
                })
                .collect(),
        )
    }
    fn flip_x(mut self) -> Self {
        for col in 0..3 {
            let tmp = self.0[2][col];
            self.0[2][col] = self.0[0][col];
            self.0[0][col] = tmp;
        }

        self
    }

    fn rotate(self) -> Self {
        let mut res = vec![vec![false; 3]; 3];
        for (i, row) in self.0.into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                // let new_row = (((i as i128) - 1) * -1 + 1) as usize;
                let new_row = (((j as i128) - 1) * -1 + 1) as usize;
                let new_col = i;
                res[new_row][new_col] = col;
            }
        }

        Present(res)
    }

    fn modify_map(&self, mut map: Vec<Vec<bool>>, x: usize, y: usize) -> Option<Vec<Vec<bool>>> {
        assert!(map[0].len() >= x + 2);
        assert!(map.len() >= y + 2);

        for (y_p, y) in (0..=2).zip(y..=y + 2) {
            for (x_p, x) in (0..=2).zip(x..=x + 2) {
                if self.0[y_p][x_p] {
                    if map[y][x] {
                        return None;
                    }
                    map[y][x] = true;
                }
            }
        }
        Some(map)
    }
}

fn part1_helper(
    mut remaining: Vec<usize>,
    map: Vec<Vec<bool>>,
    presents: &Vec<Present>,
    seen_states: &mut HashSet<Vec<Vec<bool>>>,
) -> (bool, Vec<Vec<bool>>) {
    if seen_states.contains(&map) {
        return (false, map);
    }
    seen_states.insert(map.clone());
    let mut next_present = None;
    for (i, item) in remaining.iter_mut().enumerate() {
        if *item != 0 {
            *item -= 1;
            next_present = Some(i);
            break;
        }
    }
    let next_present = match next_present {
        Some(present) => presents[present].clone(),
        None => return (true, map),
    };

    let mut present_variations = vec![
        next_present.clone(),
        next_present.clone().flip_x(),
        next_present.clone().flip_y(),
    ];
    for i in 0..3 {
        present_variations.push(present_variations[i].clone().rotate());
        present_variations.push(
            present_variations[present_variations.len() - 1]
                .clone()
                .rotate(),
        );
        present_variations.push(
            present_variations[present_variations.len() - 1]
                .clone()
                .rotate(),
        );
    }

    // println!("{remaining:?}");

    for variation in present_variations {
        // println!("variation: ");
        // println!(
        //     "{}",
        //     variation
        //         .0
        //         .iter()
        //         .map(|row| row
        //             .iter()
        //             .map(|val| if *val { "#" } else { "." })
        //             .collect::<Vec<_>>()
        //             .join(""))
        //         .collect::<Vec<_>>()
        //         .join("\n")
        // );
        for x in 0..map[0].len() - 2 {
            for y in 0..map.len() - 2 {
                // println!("{x}, {y} -> {}, {}", map[0].len(), map.len());
                if let Some(new_map) = variation.modify_map(map.clone(), x, y) {
                    let tmp = new_map.clone();
                    let res = part1_helper(remaining.clone(), new_map, presents, seen_states);

                    if res.0 {
                        println!("\n");
                        println!(
                            "{}",
                            tmp.iter()
                                .map(|line| line
                                    .iter()
                                    .map(|a| if *a { "#" } else { "." })
                                    .collect::<Vec<_>>()
                                    .join(""))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                        return res;
                    }
                }
            }
        }
    }

    // println!("failed!!");
    (false, map)
}
fn part1(inp: &(Vec<Present>, Vec<((usize, usize), Vec<usize>)>)) -> usize {
    inp.1
        .iter()
        .enumerate()
        .filter(|(i, ((size_x, size_y), present_list))| {
            println!("inital list ({i}: {present_list:?}");
            let res = part1_helper(
                present_list.clone(),
                vec![vec![false; *size_x]; *size_y],
                &inp.0,
                &mut HashSet::new(),
            );
            if res.0 {
                println!("region {i}: ");
                println!(
                    "{}",
                    res.1
                        .iter()
                        .map(|line| line
                            .iter()
                            .map(|a| if *a { "#" } else { "." })
                            .collect::<Vec<_>>()
                            .join(""))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            res.0
        })
        .count()
}
fn main() {
    let parts = include_str!("./input.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    let presents = parts[0..parts.len() - 1]
        .into_iter()
        .map(|present| {
            Present(
                present
                    .split("\n")
                    .skip(1)
                    .map(|parts| {
                        println!("{parts}");
                        parts
                            .chars()
                            .map(|ch| {
                                if ch == '#' {
                                    true
                                } else if ch == '.' {
                                    false
                                } else {
                                    panic!()
                                }
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let regions: Vec<((usize, usize), Vec<usize>)> = parts[parts.len() - 1]
        .split("\n")
        .map(|region| {
            let parts = region.split(": ").collect::<Vec<_>>();
            let tmp = parts[0].split("x").collect::<Vec<_>>();
            println!("{tmp:?}");
            (
                (tmp[0].parse().unwrap(), tmp[1].parse().unwrap()),
                parts[1]
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    // let mut present_variations = vec![
    //     presents[0].clone(),
    //     // presents[0].clone().flip_x(),
    //     // presents[0].clone().flip_y(),
    // ];
    // for i in 0..1 {
    //     present_variations.push(present_variations[i].clone().rotate());
    //     present_variations.push(
    //         present_variations[present_variations.len() - 1]
    //             .clone()
    //             .rotate(),
    //     );
    //     present_variations.push(
    //         present_variations[present_variations.len() - 1]
    //             .clone()
    //             .rotate(),
    //     );
    // }

    // for present in present_variations {
    //     println!(
    //         "{}",
    //         present
    //             .0
    //             .into_iter()
    //             .map(|row| row
    //                 .into_iter()
    //                 .map(|a| if a { "#" } else { "." })
    //                 .collect::<Vec<_>>()
    //                 .join(""))
    //             .collect::<Vec<_>>()
    //             .join("\n")
    //     );

    //     println!("\n\n");
    // }

    println!("part1: {}", part1(&(presents, regions)));
}
