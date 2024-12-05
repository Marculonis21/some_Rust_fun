use std::{collections::HashMap, fs};
use itertools::Itertools;

fn create_ordering_rules(rules: &Vec<&str>) -> Vec<Vec<i8>> {
    let mut order_matrix = vec![vec![0; 100]; 100];

    rules.iter().for_each(|rule| {
        if let Some((a,b)) = rule.split_once("|") {
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            order_matrix[a][b] = 1;
            order_matrix[b][a] = -1;
        };
    });
    
    order_matrix
}

fn check_ordering(pages: &Vec<usize>, order_matrix: &Vec<Vec<i8>>) -> bool {
    let mut a: usize;
    let mut b: usize;
    for pair in pages.iter().combinations(2) {
        (a,b) = (*pair[0],*pair[1]);

        if order_matrix[a][b] == -1 {
            return false
        }
    }

    return true
}

fn reorder(pages: &Vec<usize>, order_matrix: &Vec<Vec<i8>>) -> Vec<usize> {
    let mut new_order = vec![pages[0]];

    'next: for next_page in pages.iter().skip(1) {
        for i in 0..new_order.len() {
            if order_matrix[new_order[i]][*next_page] == -1 {
                new_order.insert(i, *next_page);
                continue 'next;
            }
        }
        new_order.push(*next_page);
    }

    println!("{:?}, {:?}", pages, new_order);

    return new_order
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.pop();

    let i = lines.iter().position(|l| *l == "").unwrap();
    let rules = lines[0..i].to_vec();
    let updates = lines[i+1..lines.len()].to_vec();

    let order_matrix = create_ordering_rules(&rules);

    let sum_middle_p1:i32 = updates.iter().map(|update| {
        let pages: Vec<usize> = update.split(",").map(|page| page.parse().unwrap()).collect();
        if check_ordering(&pages, &order_matrix) { 
            return pages[pages.len()/2] as i32
        }

        return 0
    }).sum();


    let fixed_ordering_sum_p2: i32 = updates.iter().map(|update| {
        let pages: Vec<usize> = update.split(",").map(|page| page.parse().unwrap()).collect();
        if !check_ordering(&pages, &order_matrix) { 
            let new_pages = reorder(&pages, &order_matrix);
            return new_pages[new_pages.len()/2] as i32
        }
        return 0
    }).sum();

    println!("{sum_middle_p1}");
    println!("{fixed_ordering_sum_p2}");
}

