use std::{fs, u16};

fn index_of_symbols(line: &Vec<&str>) -> Vec<u16> {
    let mut indices = vec![];

    for (i, sym) in line.iter().enumerate() {
        if !(".0123456789".contains(sym)) {
            indices.push(i as u16);
        }
    }

    indices
}

fn check_surrounding(line_num: i32,symbol_indices: i32, lines: Vec<Vec<&str>>) -> Vec<u32> {
    let mut part_numbers = vec![];

    for y in -1..2 {
        if 0 > line_num + y || line_num + y > lines.len() as i32 {
            continue;
        }

        for x in -1..2 {
            if 0 > symbol_indices + x || symbol_indices + x >= lines[0].len() as i32 {
                continue;
            }

            if x==0 && y==0 {
                continue;
            }

            // if "0123456789".contains(lines[line_num+y][1]) {
                
            // }
        }
    }

    part_numbers
}

fn search_for_parts(lines: &Vec<&Vec<&str>>) -> Vec<u32> {
    let mut part_numbers = vec![];
    let mut symbol_indices : Vec<u16>;

    for (i, line) in lines.iter().enumerate() {
        symbol_indices = index_of_symbols(line);
        if symbol_indices.is_empty() { continue; }

        for id in symbol_indices {

        }
    }

    part_numbers
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading input file");

    let mut lines_of_chars : Vec<_> = input.split("\n").map(|line| {
        let mut chars = line.split("").collect::<Vec<_>>();
        chars.pop();
        return chars
    }).collect();
    lines_of_chars.pop();

    println!("{:#?}", lines_of_chars);
}
