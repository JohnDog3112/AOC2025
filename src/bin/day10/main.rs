use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

struct Machine {
    light_config: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}
fn part1(inp: &Vec<Machine>) -> usize {
    inp.iter()
        .map(|machine| {
            let mut queue = VecDeque::new();
            struct Item {
                light_config: Vec<bool>,
                depth: usize,
            }
            queue.push_back(Item {
                light_config: vec![false; machine.light_config.len()],
                depth: 0,
            });
            let mut visited = HashSet::new();
            visited.insert(vec![false; machine.light_config.len()]);

            while let Some(next) = queue.pop_front() {
                for button in &machine.buttons {
                    let mut light_config = next.light_config.clone();
                    for light in button {
                        light_config[*light] = !light_config[*light];
                    }
                    if light_config == machine.light_config {
                        return next.depth + 1;
                    }
                    if visited.contains(&light_config) {
                        continue;
                    }
                    visited.insert(light_config.clone());
                    queue.push_back(Item {
                        light_config,
                        depth: next.depth + 1,
                    });
                }
            }
            unreachable!()
        })
        .sum()
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

#[derive(Clone, Copy, Debug)]
struct Fraction {
    num: i128,
    div: u64,
}

impl Fraction {
    fn new(num: i128, div: u64) -> Self {
        assert!(div != 0);
        Self { num, div }.simplify()
    }
    fn simplify(self) -> Self {
        let simplify_divisor = gcd(self.num.abs() as u64, self.div);
        Self {
            num: self.num / simplify_divisor as i128,
            div: self.div / simplify_divisor,
        }
    }

    fn add(self, oth: &Self) -> Self {
        let new_denominator = lcm(self.div, oth.div);
        let new_num = self.num * (new_denominator / self.div) as i128
            + oth.num * (new_denominator / oth.div) as i128;

        Self {
            num: new_num,
            div: new_denominator,
        }
        .simplify()
    }

    fn sub(self, oth: &Fraction) -> Fraction {
        let new_denominator = lcm(self.div, oth.div);
        let new_num = self.num * (new_denominator / self.div) as i128
            - oth.num * (new_denominator / oth.div) as i128;

        Fraction {
            num: new_num,
            div: new_denominator,
        }
        .simplify()
    }

    fn multiply(self, val: i128) -> Fraction {
        Fraction {
            num: self.num * val,
            div: self.div,
        }
        .simplify()
    }

    fn divide(self, val: i128) -> Fraction {
        if val > 0 {
            Fraction {
                num: self.num,
                div: self.div * (val.abs() as u64),
            }
        } else if val < 0 {
            Fraction {
                num: self.num * -1,
                div: self.div * (val.abs() as u64),
            }
        } else {
            panic!()
        }
        .simplify()
    }

    fn as_str(self) -> String {
        if self.num == 0 {
            format!("0")
        } else if self.div == 1 {
            format!("{}", self.num)
        } else {
            format!("{}/{}", self.num, self.div)
        }
    }
}
#[derive(Clone)]
struct VarRegEntry {
    vars: HashMap<usize, Fraction>,
    literal: Fraction,
}
impl VarRegEntry {
    fn new() -> Self {
        VarRegEntry {
            vars: HashMap::new(),
            literal: Fraction::new(0, 1),
        }
    }

    fn new_self(index: usize) -> Self {
        let mut tmp = Self::new();
        tmp.vars.insert(index, Fraction::new(1, 1));
        tmp
    }

    fn simplify(&mut self) {
        let mut to_remove = vec![];
        for (i, frac) in &self.vars {
            if frac.num == 0 {
                to_remove.push(*i);
            }
        }
        for to_remove in to_remove {
            self.vars.remove(&to_remove);
        }
    }
    fn multiply(&mut self, val: i128) {
        for var in &mut self.vars {
            *var.1 = var.1.multiply(val);
        }
        self.literal = self.literal.multiply(val);
        self.simplify()
    }

    fn divide(&mut self, val: i128) {
        for var in &mut self.vars {
            *var.1 = var.1.divide(val);
        }
        self.literal = self.literal.divide(val);
        self.simplify();
    }

    fn add(&mut self, oth: &VarRegEntry) {
        self.literal = self.literal.add(&oth.literal);
        for (&i, var) in &oth.vars {
            if var.num == 0 {
                continue;
            }
            if let Some(root_var) = self.vars.get_mut(&i) {
                *root_var = root_var.add(var);
            } else {
                self.vars.insert(i, *var);
            }
        }
        self.simplify()
    }

    fn subtract(&mut self, oth: &VarRegEntry) {
        self.literal = self.literal.sub(&oth.literal);
        for (&i, var) in &oth.vars {
            if var.num == 0 {
                continue;
            }
            if let Some(root_var) = self.vars.get_mut(&i) {
                *root_var = root_var.sub(var);
            } else {
                self.vars.insert(i, var.multiply(-1));
            }
        }
        self.simplify()
    }

    fn get_independent_vars(&self) -> HashSet<usize> {
        self.vars
            .iter()
            .filter_map(|(i, frac)| if frac.num == 0 { None } else { Some(*i) })
            .collect()
    }

    fn compute(&self, nums: &HashMap<usize, i128>) -> Option<Fraction> {
        Some(
            self.vars
                .iter()
                .map(|(i, frac)| {
                    if let Some(num) = nums.get(i) {
                        Some(frac.multiply(*num))
                    } else {
                        None
                    }
                })
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .fold(self.literal, |acc, next| acc.add(&next)),
        )
    }

    fn as_str(&self) -> String {
        let mut var_strs = self
            .vars
            .iter()
            .filter_map(|(index, frac)| {
                if frac.num != 0 {
                    Some((index, format!("{}*x{index}", frac.as_str())))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        var_strs.sort_by(|a, b| a.0.cmp(b.0));
        let var_strs = var_strs.into_iter().map(|(_, str)| str).collect::<Vec<_>>();
        let mut out_str = var_strs.join(" + ");
        if self.literal.num != 0 {
            if out_str.len() > 0 {
                out_str += format!(" + {}", self.literal.as_str()).as_ref();
            } else {
                out_str = self.literal.as_str();
            }
        }

        out_str
    }
}
fn part2(inp: &Vec<Machine>) -> usize {
    inp.iter()
        .enumerate()
        .map(|(_i, machine)| {
            // println!("machine: {i}");
            let mut matrix = vec![vec![0; machine.buttons.len() + 1]; machine.joltages.len()];
            for row in 0..machine.joltages.len() {
                for (i, button) in machine.buttons.iter().enumerate() {
                    for &j in button {
                        if j == row {
                            matrix[row][i] = 1;
                        }
                    }
                }
                matrix[row][machine.buttons.len()] =
                    *machine.joltages.get(row).unwrap_or(&0) as i128;
            }

            let mut main_row = 0;
            for col in 0..matrix[0].len() - 1 {
                let mut row_swap = None;
                for start_row in main_row..matrix.len() {
                    if matrix[start_row][col] != 0 {
                        row_swap = Some(start_row);
                        break;
                    }
                }

                if let Some(row_swap) = row_swap {
                    let tmp = matrix[row_swap].clone();
                    matrix[row_swap] = matrix[main_row].clone();
                    matrix[main_row] = tmp;
                } else {
                    continue;
                }

                for row in main_row + 1..matrix.len() {
                    if matrix[row][col] != 0 {
                        let modified_arr_multi = matrix[main_row][col];
                        let main_arr_multi = matrix[row][col];
                        for i_col in col..matrix[0].len() {
                            matrix[row][i_col] = matrix[row][i_col] * modified_arr_multi
                                - matrix[main_row][i_col] * main_arr_multi;
                        }
                    }
                }
                main_row += 1;
            }

            // for row in &matrix {
            //     print!("[");
            //     for (i, col) in row.into_iter().enumerate() {
            //         print!(" ");
            //         if i == matrix[0].len() - 1 {
            //             print!("| ");
            //         }
            //         print!("{:5}", col);
            //     }
            //     println!(" ]");
            // }

            let mut vars_reg = (0..matrix[0].len() - 1)
                .map(VarRegEntry::new_self)
                .collect::<Vec<_>>();
            for row in (0..matrix.len()).rev() {
                let mut var_col = None;
                for col in 0..matrix[row].len() - 1 {
                    if matrix[row][col] != 0 {
                        var_col = Some(col);
                        break;
                    }
                }
                let var_col = match var_col {
                    Some(var_col) => var_col,
                    None => continue,
                };

                let mut var_reg = VarRegEntry::new();
                var_reg.literal = Fraction::new(matrix[row][matrix[row].len() - 1], 1);
                for col in var_col + 1..matrix[row].len() - 1 {
                    let mut tmp_var_reg = vars_reg[col].clone();
                    tmp_var_reg.multiply(matrix[row][col]);
                    var_reg.subtract(&tmp_var_reg);
                }

                let divisor = matrix[row][var_col];

                var_reg.divide(divisor);

                // vars_reg[var_col] = (vars, literal);
                vars_reg[var_col] = var_reg;
            }

            // for var_reg in &vars_reg {
            //     println!("[ {:10} ]", var_reg.as_str());
            // }

            let mut sum = VarRegEntry::new();
            for var_reg in &vars_reg {
                sum.add(var_reg);
            }

            // println!("{}", sum.as_str());

            if let Some(val) = sum.compute(&HashMap::new()) {
                assert!(val.div == 1);
                // println!("\n\n\n");
                // println!("result {i}: {}", val.num);
                return val.num as usize;
            }

            let vars_set = vars_reg
                .iter()
                .flat_map(|reg| reg.get_independent_vars())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            fn recurse(
                remaining: usize,
                index: usize,
                vars: &mut HashMap<usize, i128>,
                vars_set: &Vec<usize>,
                sum: &VarRegEntry,
                vars_reg: &Vec<VarRegEntry>,
            ) -> usize {
                for reg in vars_reg {
                    if let Some(frac) = reg.compute(&vars) {
                        if frac.div != 1 || frac.num < 0 {
                            return usize::MAX;
                        }
                    }
                }

                if index == vars_set.len() {
                    if let Some(frac) = sum.compute(&vars) {
                        if frac.div != 1 || frac.num <= 0 {
                            return usize::MAX;
                        } else {
                            return frac.num as usize;
                        }
                    } else {
                        return usize::MAX;
                    }
                }

                // let mut min_vars = vars.clone();
                let mut min = usize::MAX;
                for i in 0..=remaining {
                    if let Some(val) = vars.get_mut(&vars_set[index]) {
                        *val = i as i128;
                    } else {
                        vars.insert(vars_set[index], i as i128);
                    }
                    let res = recurse(remaining - i, index + 1, vars, vars_set, sum, vars_reg);
                    if res < min {
                        min = res;
                        // min_vars = res.1;
                    }
                }
                vars.remove(&vars_set[index]);

                min
            }

            let val = recurse(
                machine.joltages.iter().sum(),
                0,
                &mut HashMap::new(),
                &vars_set,
                &sum,
                &vars_reg,
            );
            // println!("result {i}: {}", val.0);
            // for reg in vars_reg {
            //     println!("[ {:5} ]", reg.compute(&val.1).unwrap().as_str());
            // }
            // println!("\n\n\n");
            val
            // unreachable!()
        })
        .sum()
}
fn main() {
    let inp: Vec<Machine> = include_str!("./input.txt")
        .split("\n")
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            Machine {
                light_config: parts[0][1..parts[0].len() - 1]
                    .chars()
                    .map(|ch| match ch {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect(),
                buttons: parts[1..parts.len() - 1]
                    .into_iter()
                    .map(|button| {
                        button[1..button.len() - 1]
                            .split(",")
                            .map(|item| item.parse().unwrap())
                            .collect()
                    })
                    .collect(),
                joltages: parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
                    .split(",")
                    .map(|joltage| joltage.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}
