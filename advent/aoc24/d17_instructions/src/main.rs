use core::panic;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::preceded;
use nom::IResult;
use rayon::iter::WhileSome;
use std::env::args;
use std::fs;

use rayon::prelude::*;

fn register_a(input: &str) -> IResult<&str, i64> {
    preceded(tag("Register A: "), complete::i64)(input)
}
fn register_b(input: &str) -> IResult<&str, i64> {
    preceded(tag("Register B: "), complete::i64)(input)
}
fn register_c(input: &str) -> IResult<&str, i64> {
    preceded(tag("Register C: "), complete::i64)(input)
}

fn adv(ra: i64, operand: i64) -> i64 {
    let base: f64 = 2.0;
    ((ra as f64) / base.powf(operand as f64)).trunc() as i64
}

fn bxl(rb: i64, operand: i64) -> i64 {
    rb ^ operand
}

fn bst(operand: i64) -> i64 {
    operand.rem_euclid(8)
}

fn bxc(rb: i64, rc: i64) -> i64 {
    rb ^ rc
}

fn out(operand: i64) -> i64 {
    operand.rem_euclid(8)
}

fn bdv(ra: i64, operand: i64) -> i64 {
    adv(ra, operand)
}

fn cdv(ra: i64, operand: i64) -> i64 {
    adv(ra, operand)
}

fn run_machine(ra: &mut i64, rb: &mut i64, rc: &mut i64, instructions: &Vec<&str>) -> Vec<i64> {
    let mut ip: usize = 0;
    let mut output: Vec<i64> = vec![];

    loop {
        if ip > instructions.len() - 1 {
            break;
        }

        let op = instructions[ip].parse().unwrap();
        let operand = instructions[ip + 1].parse().unwrap();

        let combo_operand = match operand {
            4 => *ra,
            5 => *rb,
            6 => *rc,
            _ => operand,
        };

        // println!("IP:{} Instr:{} Op_value:{}", ip, op, operand);

        match op {
            0 => *ra = adv(*ra, combo_operand),
            1 => *rb = bxl(*rb, operand),
            2 => *rb = bst(combo_operand),
            3 => {
                if *ra != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => *rb = bxc(*rb, *rc),
            5 => output.push(out(combo_operand)),
            6 => *rb = bdv(*ra, combo_operand),
            7 => *rc = cdv(*ra, combo_operand),
            _ => panic!("We were betrayed again!"),
        }

        // move ip by 2 always
        ip += 2;
    }

    return output;
}

fn p2_search(instructions: &Vec<i64>, answer: i64, depth: usize) -> Option<i64> {
    // for a = 0:
    // b=0    b = a % 8
    // b=1    b = b ^ 0b001 
    // c=0    c = a >> b
    // b=4    b = b ^ 0b101
    // b=4    b = b ^ c
    // a=0    a = a >> 3
    // out=4  out(b % 8) # -> does not work
    //     if a != 0 jump
    
    println!("p2_search: {} {}", answer, depth);

    if instructions.len() == depth {
        return Some(answer);
    }

    let mut a: i64;
    let mut b: i64;
    let mut c: i64;

    for new_a in 0..=7 {
        // going backwards - moving old aanswer up and adding new end to the number
        a = (answer << 3) + new_a;
        // println!("tested: {} {}", new_a, a);

        b = a % 8;
        b = b ^ 1;
        c = a >> b;
        b = b ^ 5;
        b = b ^ c;

        if b % 8 == instructions[instructions.len()-(1+depth)] {
            if let Some(sol) = p2_search(instructions, a, depth+1) {
                return Some(sol);
            }

            continue;
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("copy_input.txt").expect("Problem reading file");
    
    let lines: Vec<&str> = input.split("\n").collect();

    let mut ra = register_a(lines[0]).unwrap().1;
    // let mut ra = 164541160582845; check
    let mut rb = register_b(lines[1]).unwrap().1;
    let mut rc = register_c(lines[2]).unwrap().1;

    let instructions: Vec<&str> = lines[4].split_once(" ").unwrap().1.split(",").collect();

    let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

    println!("{}", output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));

    // part2
    //
    // A = X/8
    // B = (((X%8) XOR 1) XOR 5) XOR (X / 2^((X%8) XOR 1))
    // C = X / 2^((X%8) XOR 1)

    // out -> ((((X%8) XOR 1) XOR 5) XOR (X / 2^((X%8) XOR 1))) % 8
    //
    // let goal_output = instructions.iter().map(|x| x.parse().unwrap()).collect::<Vec<i64>>(); 
    // let mut output: Vec<i64> = vec![];

    // 'ra_loop: for i in (8_i64.pow(15)..).into_iter() {
    //     if i % 10000000 == 0 {
    //         println!("processing {i}");
    //     }
    //     ra = i;
    //     output.clear();

    //     for g in goal_output.iter() {
    //         let xm8_x1 = bst(ra) ^ 1; // (X%8) XOR 1
    //         let lhs_b = xm8_x1 ^ 5;
    //         let new_c = adv(ra, xm8_x1); // X / 2^((X%8) XOR 1)
    //         let new_b = lhs_b ^ new_c;
    //         let out = out(new_b);
    //         output.push(out);

    //         if *g != out { continue 'ra_loop; }
    //         ra = adv(ra, 3);

    //         if ra == 0 { break; }
    //     }

    //     if output == goal_output {
    //         println!("Output: {i}");
    //         break;
    //     }
    // }
    //
    //
    // still bad ...
    // solve it directly?


    // let instructions: Vec<i64> = instructions.iter().map(|s| s.parse().unwrap()).collect();

    // let output = p2_search(&instructions, 0, 0);
    // println!("OUT: {:?}", output);

    // println!("Found ra: {:?}", found_ra);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in1() {
        let mut ra = 0;
        let mut rb = 0;
        let mut rc = 9;
        let instructions: Vec<&str> = vec!["2","6"];
        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(rb, 1)
    }

    #[test]
    fn in2() {
        let mut ra = 10;
        let mut rb = 0;
        let mut rc = 0;
        let instructions: Vec<&str> = vec!["5","0","5","1","5","4"];
        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(output, vec![0,1,2])
    }

    #[test]
    fn in3() {
        let mut ra = 2024;
        let mut rb = 0;
        let mut rc = 0;
        let instructions: Vec<&str> = vec!["0","1","5","4","3","0"];
        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(ra, 0);
    }

    #[test]
    fn in4() {
        let mut ra = 0;
        let mut rb = 29;
        let mut rc = 0;
        let instructions: Vec<&str> = vec!["1","7"];
        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(rb, 26);
    }

    #[test]
    fn in5() {
        let mut ra = 0;
        let mut rb = 2024;
        let mut rc = 43690;
        let instructions: Vec<&str> = vec!["4","0"];
        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(rb, 44354);
    }

    #[test]
    fn copy_self_test() {
        let mut ra = 117440;
        let mut rb = 0;
        let mut rc = 0;

        let instructions: Vec<&str> = vec!["0","3","5","4","3","0"];

        let output = run_machine(&mut ra, &mut rb, &mut rc, &instructions);

        assert_eq!(output, vec![0,3,5,4,3,0]);
    }
}
