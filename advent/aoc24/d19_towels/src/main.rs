use std::fs;
use std::collections::HashMap;

fn parse_input(input: &String) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.split("\n").collect();

    let (towels, patterns) = lines.split_at(lines.iter().position(|line| *line == "").unwrap());

    assert_eq!(towels.len(), 1, "towels should only be one line");

    let towels: Vec<&str> = towels[0].split(",").map(|towel| towel.trim()).collect();
    let patterns: Vec<&str> = patterns.to_vec().into_iter().filter(|line| *line != "").collect();
    // println!("{towels:#?}\n {patterns:#?}");

    (towels, patterns)
}

fn dfs_pattern(pattern: String, towels: &Vec<&str>) -> bool {
    if pattern == "" { return true; }

    for t in towels {
        if pattern.len() < t.len() { continue; }

        let (l, rest) = pattern.split_at(t.len());
        if *t == l {
            if dfs_pattern(rest.to_string(), towels) {
                return true;
            }
        }
    }

    return false;
}

fn dfs_pattern_occurrence(memory: &mut HashMap<String, i64>, pattern: String, towels: &Vec<&str>) -> i64 {
    if pattern == "" { return 1; }
    if memory.contains_key(&pattern) {
        return memory[&pattern];
    }

    let mut sum: i64 = 0;
    for t in towels {
        if pattern.len() < t.len() { continue; }

        let (l, rest) = pattern.split_at(t.len());
        if *t == l {
            sum += dfs_pattern_occurrence(memory, rest.to_string(), towels);
        }
    }

    memory.insert(pattern, sum);

    return sum;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");

    let (towels, patterns) = parse_input(&input);

    let do_able_p1 = patterns.iter().filter(|pattern| dfs_pattern(pattern.to_string(), &towels)).count();
    println!("Part1: {}", do_able_p1);

    let mut memory: HashMap<String, i64> = HashMap::new();
    let possible_ways: i64 = patterns.iter().map(|pattern| dfs_pattern_occurrence(&mut memory, pattern.to_string(), &towels)).sum();

    println!("Part2: {}", possible_ways);
}
