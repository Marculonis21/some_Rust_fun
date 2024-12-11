use std::{fs, usize, vec};
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn get_surround(&self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let mut surround: Vec<Pos> = vec![];

        if self.x+1 < max_x {
            surround.push(Pos{x:self.x+1, y:self.y})
        }
        if self.x > 0 {
            surround.push(Pos{x:self.x-1, y:self.y})
        }

        if self.y+1 < max_y {
            surround.push(Pos{x:self.x, y:self.y+1})
        }
        if self.y > 0 {
            surround.push(Pos{x:self.x, y:self.y-1})
        }

        return surround
    }
}

fn get_starters(map: &Vec<Vec<i8>>) -> Vec<Pos> {
    let mut starters: Vec<Pos> = vec![];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                starters.push(Pos{x,y});
            }
        }
    }

    return starters
}

// dfs basically
fn go_on_trail(map: &Vec<Vec<i8>>, pos: Pos, found: &mut HashSet<Pos>) -> i32 {
    if map[pos.y][pos.x] == 9 { 
        if !found.contains(&pos) {
            found.insert(pos);

            return 1;
        }

        return 0; 
    }

    let mut successful = 0;
    for next in pos.get_surround(map[0].len(), map.len()) {
        if map[pos.y][pos.x]+1 == map[next.y][next.x] {
            successful += go_on_trail(map, next, found);
        }
    }

    return successful;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");

    let map: Vec<Vec<i8>> = input.split("\n").filter(|line| *line != "")
                                 .map(|c| c.split("").filter(|x| *x != "")
                                           .map(|x| x.parse().unwrap()).collect()).collect();

    let starters = get_starters(&map);
    let result_p1: i32 = starters.iter().map(|s| go_on_trail(&map, s.clone(), &mut HashSet::new())).sum(); 

    println!("Part1: {}", result_p1);
}
