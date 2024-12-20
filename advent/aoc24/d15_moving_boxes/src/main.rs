use core::panic;
use std::{fs, io::stdin, usize};

#[derive(Clone,Debug)]
struct Pos {
    x: i32,
    y: i32
}

type Map<'a > = Vec<Vec<&'a str>>;

fn parse_input(input: &String) -> (Map, Vec<&str>) {
    let lines: Vec<&str> = input.split("\n").collect();

    let (map, moves) = lines.split_at(lines.iter().position(|line| *line == "").unwrap());
    let map: Vec<Vec<&str>> = map.iter().map(|line| line.split("").filter(|c| *c != "").collect()).collect();
    let moves: Vec<&str> = moves.iter().map(|line| line.split("").filter(|c| *c != "")).flatten().collect();

    (map, moves)
}

fn find_robot(map: &Map) -> Option<Pos> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == "@" {
                return Some(Pos{x:x as i32,y:y as i32})
            }
        }
    }

    None
}

fn move_in_dir_p1(pos: &Pos, next: &Pos, map: &mut Map) -> bool {
    if pos.y == 0 {
        return false;
    }
    
    let dx = next.x-pos.x;
    let dy = next.y-pos.y;

    match map[next.y as usize][next.x as usize] {
        "." => {
            (map[next.y as usize][next.x as usize], map[pos.y as usize][pos.x as usize]) = (map[pos.y as usize][pos.x as usize], map[next.y as usize][next.x as usize]);
            return true;
        }
        "#" => {
            return false;
        }
        "O" => {
            if move_in_dir_p1(next, &Pos{x:next.x+dx, y:next.y+dy}, map) {
                (map[next.y as usize][next.x as usize], map[pos.y as usize][pos.x as usize]) = (map[pos.y as usize][pos.x as usize], map[next.y as usize][next.x as usize]);
                return true
            }
            return false
        }
        _ => panic!("Unknown map symbol")
    }
}

fn switch_map(map: &mut Map, p1: &Pos, p2: &Pos) {
    println!("moving1 - {:?} - {}",p1,map[p1.y as usize][p1.x as usize]);
    println!("moving2 - {:?} - {}",p2,map[p2.y as usize][p2.x as usize]);

    (map[p1.y as usize][p1.x as usize], map[p2.y as usize][p2.x as usize]) = (map[p2.y as usize][p2.x as usize], map[p1.y as usize][p1.x as usize])
}

fn move_in_dir_p2(pos: &Pos, next: &Pos, map: &mut Map, do_move: bool) -> bool {
    if pos.y == 0 {
        return false;
    }
    
    let dx = next.x-pos.x;
    let dy = next.y-pos.y;

    match map[next.y as usize][next.x as usize] {
        "#" => {
            return false;
        }

        "." => {
            if do_move {
                switch_map(map, pos, next);
            }

            return true;
        }

        s @ "[" | s @ "]" => {
            if dy == 0 {
                if move_in_dir_p2(next, &Pos{x:next.x+dx, y:next.y+dy}, map, true) {
                    switch_map(map, pos, next);
                    return true
                }
            }
            else {
                if s == "[" {
                    let next_r = Pos{x:next.x+1, y:next.y}; 
                    let pos_r = Pos{x:pos.x+1, y:pos.y}; 
                    if move_in_dir_p2(next, &Pos{x:next.x+dx, y:next.y+dy}, map, false) && 
                       move_in_dir_p2(&next_r, &Pos{x:next_r.x+dx, y:next_r.y+dy}, map, false) {

                       move_in_dir_p2(next, &Pos{x:next.x+dx, y:next.y+dy}, map, do_move);
                       move_in_dir_p2(&next_r, &Pos{x:next_r.x+dx, y:next_r.y+dy}, map, do_move);

                       if do_move {
                           switch_map(map, pos, next);
                           if map[pos.y as usize][pos.x as usize] == "@"  {
                               switch_map(map, &pos_r, &next_r);
                           }
                       }

                       return true;
                    }
                }
                else {
                    let next_l = Pos{x:next.x-1, y:next.y}; 
                    let pos_l = Pos{x:pos.x-1, y:pos.y}; 
                    if move_in_dir_p2(next, &Pos{x:next.x+dx, y:next.y+dy}, map, false) && 
                       move_in_dir_p2(&next_l, &Pos{x:next_l.x+dx, y:next_l.y+dy}, map, false) {

                       move_in_dir_p2(next, &Pos{x:next.x+dx, y:next.y+dy}, map, do_move);
                       move_in_dir_p2(&next_l, &Pos{x:next_l.x+dx, y:next_l.y+dy}, map, do_move);

                       if do_move {
                           switch_map(map, pos, next);
                           if map[pos.y as usize][pos.x as usize] == "@"  {
                               switch_map(map, &pos_l, &next_l);
                           }
                       }
                       return true;
                    }
                }

                return false;
            }
            return false
        }

        _ => panic!("Unknown map symbol")
    }
}

fn print_map(map: &Map) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}",map[y][x]);
        }
        print!("\n");
    }
}

fn calc_map(map: &Map, box_str: &str) -> u64 {
    let mut sum: u64 = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == box_str {
                sum += 100*(y as u64) + (x as u64);
            }
        }
    }

    sum
}

fn make_map_chonkier(map: &mut Map) {
    let size = map.len();

    for _ in 0..size {
        let mut new_line = vec![""; map[0].len()*2];

        for x in 0..map[0].len() {
            match map[0][x] {
                "#" | "." => {
                    new_line[x*2] = map[0][x];
                    new_line[x*2 + 1] = map[0][x];
                }
                "O" => {
                    new_line[x*2] = "[";
                    new_line[x*2 + 1] = "]"
                }
                "@" => {
                    new_line[x*2] = "@";
                    new_line[x*2 + 1] = "."
                }
                _ => panic!("Unknown symbol while making chonkier map")
            }
        }

        map.push(new_line);
        map.remove(0);
    }
}


fn solution(map: &mut Map, moves: &Vec<&str>) {
    let stdin = stdin();
    let mut r_pos = find_robot(&map).unwrap();

    let mut next_move: Pos;
    for m in moves {
    // loop {
        // let mut s: String = "".into();
        // let _ = stdin.read_line(&mut s);
        // s = s.trim().to_string();
        // println!("{}", s);
        // next_move = match s.to_string().as_str() {
        next_move = match m.to_string().as_str() {
            "^" => Pos{x: r_pos.x, y: r_pos.y-1},
            "v" => Pos{x: r_pos.x, y: r_pos.y+1},
            ">" => Pos{x: r_pos.x+1, y: r_pos.y},
            "<" => Pos{x: r_pos.x-1, y: r_pos.y},
            _ => panic!("Unknown move")
        };
        // if move_in_dir_p1(&r_pos, &next_move, map) {
        //     r_pos = next_move.clone();
        // }
        if move_in_dir_p2(&r_pos, &next_move, map, true) {
            r_pos = next_move.clone();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input_test.txt").expect("Problem reading file");

    // let (mut map, moves) = parse_input(&input);
    // print_map(&map);
    // solution(&mut map, &moves);
    // let coord_sum = calc_map(&map);
    // println!("Part1 {}", coord_sum);

    let (mut map, moves) = parse_input(&input);
    print_map(&map);
    make_map_chonkier(&mut map);
    solution(&mut map, &moves);
    print_map(&map);
    let coord_sum = calc_map(&map, "[");
    println!("Part2 {}", coord_sum);
}
