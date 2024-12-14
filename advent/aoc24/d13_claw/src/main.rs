use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;
use core::f64;
use std::collections::binary_heap::Iter;
use std::fs;
use std::slice;

#[derive(Default)]
struct Problem {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

impl Problem {
    fn solve(mut self, p2: bool) -> Option<(u64,u64)> {
        if p2 {
            self.px += 10000000000000.0;
            self.py += 10000000000000.0;
        }

        let ca: f64 = (self.px*self.by - self.py*self.bx) / (self.ax * self.by - self.ay * self.bx);
        let cb: f64 = (self.px - self.ax * ca) / self.bx;

        println!("{}, {}", ca,cb);
        if ca.fract() == 0.0 && cb.fract() == 0.0 {
            return Some((ca as u64, cb as u64));
        }

        None
    }
}

fn get_button_A(line: &str) -> IResult<&str, (u64,u64)> {
    preceded(
        tag("Button A: X+"),
        separated_pair(complete::u64, 
                       tag(", Y+"), 
                       complete::u64)
    )(line)
}

fn get_button_B(line: &str) -> IResult<&str, (u64,u64)> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::u64, 
                       tag(", Y+"), 
                       complete::u64),
    )(line)
}

fn get_end(line: &str) -> IResult<&str, (u64,u64)> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::u64, 
                       tag(", Y="), 
                       complete::u64),
    )(line)
}

fn split_to_subproblems(input: &String) -> Vec<Problem> {
    input
        .split("\n")
        .filter(|line| *line != "")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|p| {
            let a = get_button_A(p[0]).unwrap().1;
            let b = get_button_B(p[1]).unwrap().1;
            let p = get_end(p[2]).unwrap().1;

            Problem{ax: a.0 as f64, ay: a.1 as f64,
                    bx: b.0 as f64, by: b.1 as f64,
                    px: p.0 as f64, py: p.1 as f64}

        }).collect()
}

fn solve(path: String, p2: bool) -> u64 {
    let input = fs::read_to_string(path).expect("Problem reading file");

    let mut cost = 0;
    for problem in split_to_subproblems(&input) {
        if let Some((a,b)) = problem.solve(p2) {
            println!("{},{}", a,b);
            cost += 3*a + 1*b;
        }
    }

    println!("Problem cost: {cost}");

    return cost
}

fn main() {
    solve("input.txt".into(), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input() {
        assert_eq!(solve("small_input.txt".into()), 480);
    }
}
