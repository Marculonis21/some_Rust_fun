use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Equation {
    expected_result: i64,
    operands: Vec<i64> 
}

pub struct PermutationsReplacementIter<I> {
    items: Vec<I>,
    permutation: Vec<usize>,
    group_len: usize,
    finished: bool,
}

impl<I: Copy> PermutationsReplacementIter<I> {
    fn increment_permutation(&mut self) -> bool {
        let mut idx = 0;

        loop {
            if idx >= self.permutation.len() {
                return true;
            }

            self.permutation[idx] += 1;

            if self.permutation[idx] >= self.items.len() {
                self.permutation[idx] = 0;
                idx += 1;
            } else {
                return false;
            }
        }
    }

    fn build_vec(&self) -> Vec<I> {
        let mut vec = Vec::with_capacity(self.group_len);

        for idx in &self.permutation {
            vec.push(self.items[*idx]);
        }

        vec
    }
}

impl<I: Copy> Iterator for PermutationsReplacementIter<I> {
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let item = self.build_vec();

        if self.increment_permutation() {
            self.finished = true;
        }

        Some(item)
    }
}

pub trait ToPermutationsWithReplacement {
    type Iter;
    fn permutations_with_replacement(self, group_len: usize) -> Self::Iter;
}

impl<I: Iterator> ToPermutationsWithReplacement for I {
    type Iter = PermutationsReplacementIter<<I as Iterator>::Item>;

    fn permutations_with_replacement(self, group_len: usize) -> Self::Iter {
        let items = self.collect::<Vec<_>>();
        PermutationsReplacementIter {
            permutation: vec![0; group_len],
            group_len,
            finished: group_len == 0 || items.len() == 0,
            items,
        }
    }
}

impl Equation {
    fn new(result: i64, operands: Vec<i64>) -> Self{
        Self{expected_result:result, operands}
    }

    fn try_solve(&self) -> Option<i64> {
        let tests = vec!['+','*'].into_iter().permutations_with_replacement(self.operands.len()-1);

        let mut result: i64;
        for ops in tests {

            result = self.operands[0];
            for op in 0..ops.len() {
                match ops[op] {
                    '+' => result += self.operands[op+1],
                    '*' => result *= self.operands[op+1],
                    _ => ()
                }
            }

            // println!("{},{:?}", result, ops);

            if result == self.expected_result {
                return Some(result);
            }
        }

        None
    }

    fn try_solve_p2(&self) -> Option<i64> {
        let tests = vec!['+','*', '|'].into_iter().permutations_with_replacement(self.operands.len()-1);

        let mut result: i64;
        for ops in tests {

            result = self.operands[0];
            for op in 0..ops.len() {
                match ops[op] {
                    '+' => result += self.operands[op+1],
                    '*' => result *= self.operands[op+1],
                    '|' => result = (result.to_string() + &self.operands[op+1].to_string()).parse::<i64>().unwrap(),
                    _ => ()
                }
            }

            // println!("{},{:?}", result, ops);

            if result == self.expected_result {
                return Some(result);
            }
        }

        None
    }
}


fn parse_equation(line: &str) -> Equation {
    let (results_s, parts_s) = line.split_once(": ").unwrap();

    let result = results_s.parse::<i64>().unwrap();
    let parts: Vec<i64> = parts_s.trim().split(" ").map(|n| n.parse::<i64>().unwrap()).collect();

    Equation::new(result, parts)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problems opening the file");
    // let input = fs::read_to_string("small_input.txt").expect("Problems opening the file");
    let lines: Vec<&str> = input.split("\n").filter(|line| *line != "").collect();

    let equations: Vec<Equation> = lines.iter().map(|line| parse_equation(line)).collect();

    let result: i64 = equations.iter().map(|eq| {
        if let Some(value) = eq.try_solve() {
            // println!("{:#?}",eq);
            return value
        }
        return 0
    }).sum();

    let result2: i64 = equations.iter().map(|eq| {
        if let Some(value) = eq.try_solve_p2() {
            // println!("{:#?}",eq);
            return value
        }
        return 0
    }).sum();

    println!("Part1: {}", result);
    println!("Part2: {}", result2);
}
