use std::collections::HashMap;
use std::fs;
use std::num;
use std::collections::hash_map;


fn strings_to_ints(pair: (&str, &str)) -> (i32, i32) {
    let (a, b) = pair;
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

fn get_lists(input: &String) -> (Vec<i32>, Vec<i32>) {
    // idea: get columns into their own list, sort them and then traverse both 
    // one by one getting the differences
    let mut list_l: Vec<i32> = vec![];
    let mut list_r: Vec<i32> = vec![];

    for line in input.split("\n") {
        match line.split_once(" ") {
            Some(pair) => {
                let (a,b) = strings_to_ints(pair);
                list_l.push(a);
                list_r.push(b);
            }
            None => continue,
        }
    }

    (list_l, list_r)
}

fn part1(input: &String) {
    let (mut list_l, mut list_r) = get_lists(input);

    list_l.sort();
    list_r.sort();

    let mut sum: i32 = 0;
    // UHH There is an equiv for a python zip! neat
    for (a,b) in list_l.iter().zip(list_r) {
        sum += i32::abs(a - b);
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &String) {
    let (list_l, list_r) = get_lists(input);

    // frequency hashmap
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for num in list_r.iter() {
        // THIS syntax is interesting!
        freq.entry(*num).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut sum_similarity_score: i32 = 0;
    for a in list_l.iter() {
                                     // stars everywhere
        sum_similarity_score += a * (*freq.entry(*a).or_insert(0));
    }

    println!("Part 2: {}", sum_similarity_score);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input Problem");

    part1(&input);
    part2(&input);
}







