use core::panic;
use std::fs;
use regex::Regex;

fn part1(input: &String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let first = &cap[1].parse::<i32>().unwrap();
        let second = &cap[2].parse::<i32>().unwrap();

        sum += first*second;
    }
    sum
}

fn clean_input(input: &String) -> String {
    // clean input of all blocks of text from don't to do
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    re.replace_all(&input, "").to_string()
}

fn part2(input: &String) -> i32{
    let clean = clean_input(&input);
    part1(&clean)
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Input Problem").replace("\n", "");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
