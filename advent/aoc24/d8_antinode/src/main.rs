use std::{fs, usize};
use std::collections::HashSet;
use itertools::Itertools;


fn get_all_symbol_pos(map: &Vec<Vec<&str>>, symbol: &str) -> HashSet<(i32,i32)> {
    let mut pos: HashSet<(i32,i32)> = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == symbol {
                pos.insert((x as i32,y as i32));
            }
        }
    }

    return pos
}

fn get_all_pairs(map: &Vec<Vec<&str>>) -> HashSet<((i32,i32), (i32,i32))> {
    let mut pair_pos: HashSet<((i32,i32), (i32,i32))> = HashSet::new(); 

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == "." { continue; }

            let symbol_positions = get_all_symbol_pos(map, map[y][x]); 
            symbol_positions.into_iter().combinations(2).for_each(|pair| 
            {
                pair_pos.insert((pair[0], pair[1])); // im pretty sure this is correct
            });
        }
    }

    return pair_pos
}

fn get_all_antinodes(mut map: Vec<Vec<&str>>, pair_pos: &HashSet<((i32,i32), (i32,i32))>) -> i32 {
    let mut a: (i32,i32);
    let mut b: (i32,i32);
    let mut anti_a: (i32,i32);
    let mut anti_b: (i32,i32);

    let mut dif_x: i32;
    let mut dif_y: i32;

    let mut unique_antinodes = 0;

    for pair in pair_pos {
        a = pair.0;
        b = pair.1;

        dif_x = b.0 - a.0;
        dif_y = b.1 - a.1;

        anti_a = (a.0 - dif_x, a.1 - dif_y);
        anti_b = (b.0 + dif_x, b.1 + dif_y);

        if (anti_a.0 as usize) >= 0 && (anti_a.0 as usize) < map[0].len() &&
           (anti_a.1 as usize) >= 0 && (anti_a.1 as usize) < map.len() {
            if map[anti_a.1 as usize][anti_a.0 as usize] != "#"{
                map[anti_a.1 as usize][anti_a.0 as usize] = "#"; 
                unique_antinodes += 1;
            }
        }
        if (anti_b.0 as usize) >= 0 && (anti_b.0 as usize) < map[0].len() &&
           (anti_b.1 as usize) >= 0 && (anti_b.1 as usize) < map.len() {

            if map[anti_b.1 as usize][anti_b.0 as usize] != "#"{
                map[anti_b.1 as usize][anti_b.0 as usize] = "#"; 
                unique_antinodes += 1;
            }
        }
    }

    return unique_antinodes;
}

fn get_all_antinodes_p2(mut map: Vec<Vec<&str>>, pair_pos: &HashSet<((i32,i32), (i32,i32))>) -> i32 {
    let mut a: (i32,i32);
    let mut b: (i32,i32);
    let mut anti_a: (i32,i32);
    let mut anti_b: (i32,i32);

    let mut dif_x: i32;
    let mut dif_y: i32;

    let mut unique_antinodes = 0;

    for pair in pair_pos {
        a = pair.0;
        b = pair.1;

        dif_x = b.0 - a.0;
        dif_y = b.1 - a.1;

        let mut i: i32 = -1;
        let mut a_done = false;
        let mut b_done = false;
        while !(a_done && b_done){
            i += 1;
            anti_a = (a.0 - dif_x*i, a.1 - dif_y*i);
            anti_b = (b.0 + dif_x*i, b.1 + dif_y*i);

            if (anti_a.0 as usize) >= 0 && (anti_a.0 as usize) < map[0].len() &&
                (anti_a.1 as usize) >= 0 && (anti_a.1 as usize) < map.len() {
                if map[anti_a.1 as usize][anti_a.0 as usize] != "#"{
                    map[anti_a.1 as usize][anti_a.0 as usize] = "#"; 
                    unique_antinodes += 1;
                }
            }
            else {
                a_done = true;
            }

            if (anti_b.0 as usize) >= 0 && (anti_b.0 as usize) < map[0].len() &&
                (anti_b.1 as usize) >= 0 && (anti_b.1 as usize) < map.len() {

                if map[anti_b.1 as usize][anti_b.0 as usize] != "#"{
                    map[anti_b.1 as usize][anti_b.0 as usize] = "#"; 
                    unique_antinodes += 1;
                }
            }
            else {
                b_done = true;
            }
        }
    }

    // for y in 0..map.len() {
    //     println!();
    //     for x in 0..map[y].len() {
    //         print!("{}", map[y][x]);
    //     }
    // }
    return unique_antinodes;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    let map: Vec<Vec<&str>> = input.split("\n").filter(|line| *line != "").map(|line| line.split("").filter(|c| *c != "").collect()).collect();

    let antinode_pos: HashSet<(i32, i32)> = HashSet::new();

    let all_pairs = get_all_pairs(&map);
    let result_p1 = get_all_antinodes(map.clone(), &all_pairs);
    let result_p2 = get_all_antinodes_p2(map.clone(), &all_pairs);
    println!("Part1 {}", result_p1);
    println!("Part2 {}", result_p2);
}
