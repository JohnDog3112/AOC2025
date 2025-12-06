#[derive(Debug, Clone)]
struct Range {
    min: usize,
    max: usize,
}
impl Range {
    fn num_in_range(&self, num: usize) -> bool {
        self.min <= num && num <= self.max
    }
}

#[derive(Debug, Clone)]
struct Input {
    ranges: Vec<Range>,
    ids: Vec<usize>,
}

fn part1(inp: &Input) -> usize {
    inp.ids
        .iter()
        .filter(|id| inp.ranges.iter().any(|range| range.num_in_range(**id)))
        .count()
}

fn part2(inp: &Input) -> usize {
    let mut true_ranges: Vec<Range> = vec![];
    'outer: for range in &inp.ranges {
        let mut min_range_override = None;
        let mut max_range_override = None;
        let mut fully_contained = vec![];

        for (i, true_range) in true_ranges.iter().enumerate() {
            let true_range_min = true_range.num_in_range(range.min);
            let true_range_max = true_range.num_in_range(range.max);

            let range_min = range.num_in_range(true_range.min);
            let range_max = range.num_in_range(true_range.max);

            if true_range_min && true_range_max {
                //fully contained, skip
                continue 'outer;
            }

            if range_min && range_max {
                // this "true_range" is fully contained, add it to a list to be removed
                fully_contained.push(i);
                continue;
            }

            if range_min {
                // min overlap
                assert!(min_range_override == None);
                min_range_override = Some(i);
            }

            if range_max {
                // max overlap
                assert!(max_range_override == None);
                max_range_override = Some(i);
            }
        }

        match (min_range_override, max_range_override) {
            // no overlaps (other than fully contained)
            (None, None) => {
                true_ranges.push(range.clone());
            }
            // extend maximum range at this index
            (None, Some(max_index)) => {
                true_ranges[max_index].max = range.max;
            }
            // extend minimum range at this index
            (Some(min_index), None) => {
                true_ranges[min_index].min = range.min;
            }
            // Bridging two ranges together
            (Some(min_index), Some(max_index)) => {
                // override min_index, delete max_index
                true_ranges[min_index].min = true_ranges[max_index].min;

                fully_contained.push(max_index);
                fully_contained.sort();
            }
        }

        for index in fully_contained.into_iter().rev() {
            true_ranges.remove(index);
        }
    }

    true_ranges
        .into_iter()
        .map(|range| range.max - range.min + 1)
        .sum()
}

fn main() {
    let inp: Vec<&str> = include_str!("./input.txt").split("\n\n").collect();

    let inp = Input {
        ranges: inp[0]
            .split("\n")
            .map(|line| {
                let parts: Vec<_> = line.split("-").collect();
                Range {
                    min: parts[0].parse().unwrap(),
                    max: parts[1].parse().unwrap(),
                }
            })
            .collect(),
        ids: inp[1]
            .split("\n")
            .map(|line| line.parse().unwrap())
            .collect(),
    };

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
