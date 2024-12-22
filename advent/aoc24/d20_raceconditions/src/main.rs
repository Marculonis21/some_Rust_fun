use std::fs;

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type Map = Vec<Vec<bool>>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn expand(&self, map: &Map) -> Vec<Self> {
        let mut next: Vec<Pos> = vec![];

        if self.x < map[0].len()-1 && map[self.y][self.x+1] {
            next.push(Pos{x:self.x+1, y:self.y});
        }
        if self.x > 0 && map[self.y][self.x-1] {
            next.push(Pos{x:self.x-1, y:self.y});
        }
        if self.y < map.len()-1 && map[self.y+1][self.x] {
            next.push(Pos{x:self.x, y:self.y+1});
        }
        if self.y > 0 && map[self.y-1][self.x] {
            next.push(Pos{x:self.x, y:self.y-1});
        }

        return next;
    }

    fn dist(p1: &Pos, p2: &Pos) -> i64 {
        return i64::abs(p1.x as i64 - p2.x as i64) + i64::abs(p1.y as i64 - p2.y as i64);
    }
}

fn parse_input(input: &String) -> (Map, Pos, Pos) {
    let char_map: Vec<Vec<&str>> = input.split("\n").filter(|line| *line != "").map(|line| line.split("").filter(|c| *c != "").collect()).collect();
    let mut map: Map = vec![vec![false; char_map[0].len()]; char_map.len()];

    let mut start = Pos::default();
    let mut end = Pos::default();

    for y in 0..char_map.len() {
        for x in 0..char_map[y].len() {
            match char_map[y][x] {
                "S" => {
                    start = Pos{x,y}; 
                    map[y][x] = true;
                }
                "E" => {
                    end   = Pos{x,y};
                    map[y][x] = true;
                }
                "." => map[y][x] = true,
                _ => ()
            }
        }
    }

    (map, start, end)
}

fn number_map(map: &Map, start: &Pos, end: &Pos) -> HashMap<Pos, i64> {
    let mut track: HashMap<Pos, i64> = HashMap::new();

    let mut timer = 0;

    let mut next: Option<Pos> = Some(start.clone());

    'big: while let Some(node) = &next {
        track.insert(node.clone(), timer);
        timer += 1;

        let more = node.expand(&map);
        assert!(more.len() <= 2 , "I thought that it is always like that...");

        for m in more {
            if !track.contains_key(&m) {
                next = Some(m.clone());
                continue 'big;
            }
        }
        next = None;
    }

    return track;
}

fn print_map(map: &Map, numbering: &HashMap<Pos,i64>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                // 2 wide for the numbers to fit... xD
                print!("{:2}",numbering[&Pos{x,y}]);
            } else {
                print!("__");
            }
        }
        print!("\n");
    }
}

fn find_shortcuts(numbering: &HashMap<Pos, i64>, cheat_duration: i64, time_saved_min: i64) -> HashSet<(Pos, Pos, i64)> {
    let mut shortcuts: HashSet<(Pos, Pos, i64)> = HashSet::new();

    let len = numbering.keys().len();
    let mut i = -1;
    for pos in numbering.keys() {
        i += 1;
        // println!("{}/{}", i, len);
        for other in numbering.keys() {
            if other == pos {continue;}
            if numbering[other] < numbering[pos] {continue;}

            let dist = Pos::dist(&pos, &other);

            if dist < (cheat_duration+1) {
                let time_save = numbering[other] - numbering[pos] - dist;

                if time_save >= time_saved_min {
                    shortcuts.insert((pos.clone(),other.clone(),time_save));
                }
            }
        }
    }

    return shortcuts;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");

    let (map, start, end) = parse_input(&input);

    let numbering = number_map(&map, &start, &end);

    // print_map(&map, &numbering);

    let shortcuts = find_shortcuts(&numbering, 2, 100);
    println!("Part1: {}", shortcuts.len());

    let shortcuts = find_shortcuts(&numbering, 20, 100);
    println!("Part2: {}", shortcuts.len());
}
