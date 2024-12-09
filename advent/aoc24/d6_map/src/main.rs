use core::panic;
use std::{fs, usize};
use std::collections::HashSet;

#[derive(Debug,PartialEq,Clone,Eq,Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

enum Sensor {
    Free,
    Wall,
    End,
}

#[derive(Clone)]
struct Player {
    x: usize,
    y: usize,
    facing: Dir,
}

trait ToSensor {
    fn get_sensor_data(&self) -> Sensor;
}

impl ToSensor for str {
    fn get_sensor_data(&self) -> Sensor {
       match self {
            "." | "X" | "^" | "v" | "<" | ">" => Sensor::Free,
            "#" | "O" => Sensor::Wall,
            _ => {
                println!("{}",self);
                panic!("WEIRD MAP SYMBOL")
            }
        }
    }
}

impl Player {
    fn new(pos: (usize,usize), facing: Dir) -> Self {
        Self {x:pos.0, y:pos.1, facing:facing}
    }

    fn move_forward(&mut self) {
        match self.facing {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }

    fn rotate(&mut self) {
        match self.facing {
            Dir::Up => self.facing = Dir::Right,
            Dir::Down => self.facing = Dir::Left,
            Dir::Left => self.facing = Dir::Up,
            Dir::Right => self.facing = Dir::Down,
        }
    }

    fn sensor(&self, map: &Vec<Vec<&str>>) -> Sensor {
        match self.facing {
            Dir::Up => {
                if self.y == 0 { return Sensor::End }
                return map[self.y-1][self.x].get_sensor_data()
            }
            Dir::Down => {
                if self.y == map.len()-1 { return Sensor::End }
                return map[self.y+1][self.x].get_sensor_data()
            }
            Dir::Left => {
                if self.x == 0 { return Sensor::End }
                return map[self.y][self.x-1].get_sensor_data()
            }
            Dir::Right => {
                if self.x == map[0].len()-1 { return Sensor::End }
                return map[self.y][self.x+1].get_sensor_data()
            }
        }
    }
}

fn get_starting_pos(map: &Vec<Vec<&str>>) -> Option<Player>{
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                "^" => return Some(Player::new((x,y), Dir::Up)),
                ">" => return Some(Player::new((x,y), Dir::Right)),
                "<" => return Some(Player::new((x,y), Dir::Left)),
                "v" => return Some(Player::new((x,y), Dir::Down)),
                _ => ()
            }
        }
    }

    eprintln!("No starting place was found!");
    None
}

fn print_map(map: &Vec<Vec<&str>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}",map[y][x]);
        }
        println!();
    }
}

fn print_history(map: &Vec<Vec<&str>>, history: &HashSet<(usize,usize,Dir)>) {
    println!();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if history.contains(&(x,y,Dir::Up)) {
                print!("^");
            }
            else if history.contains(&(x,y,Dir::Down)) {
                print!("v");
            }
            else if history.contains(&(x,y,Dir::Left)) {
                print!("<");
            }
            else if history.contains(&(x,y,Dir::Right)) {
                print!(">");
            }
            else {
                print!("{}",map[y][x]);
            }
        }
        println!();
    }
}

fn run_sim(mut player: Player, mut map: Vec<Vec<&str>>) -> i32 {
    let mut visited: i32 = 1;
    map[player.y][player.x] = "X";

    loop {
        if map[player.y][player.x] == "." {
            map[player.y][player.x] = "X";
            visited += 1;
        }

        match player.sensor(&map) {
            Sensor::End => return visited, 
            Sensor::Free => player.move_forward(),
            Sensor::Wall => player.rotate(),
        }
    }
}

fn run_sim_until_pos(mut player: Player, map: &Vec<Vec<&str>>) -> bool {
    let mut history: HashSet<(usize,usize,Dir)> = HashSet::new();

    loop {
        if history.contains(&(player.x, player.y, player.facing.clone())) {
            // print_history(&map, &history);
            return true;
        }

        history.insert((player.x, player.y, player.facing.clone()));
        match player.sensor(&map) {
            Sensor::End => return false,
            Sensor::Free => player.move_forward(),
            Sensor::Wall => player.rotate(),
        }
    }
}
// fn run_sim_p2(mut player: Player, map: Vec<Vec<&str>>) -> i32 {
//     let starting_pos = (player.x, player.y);
//     let mut obstacles: Vec<(usize, usize)> = vec![]; 
//     let mut can_loop: i32 = 0;

//     loop {
//         match player.sensor(&map) {
//             Sensor::End => return can_loop, 
//             Sensor::Free => {
//                 let mut test_map = map.clone();
//                 let new_obstacle: (usize, usize);

//                 match player.facing {
//                     Dir::Up => {
//                         test_map[player.y-1][player.x] = "O";
//                         new_obstacle = (player.x, player.y-1);
//                     }
//                     Dir::Down => {
//                         test_map[player.y+1][player.x] = "O";
//                         new_obstacle = (player.x, player.y+1);
//                     }
//                     Dir::Left => {
//                         test_map[player.y][player.x-1] = "O";
//                         new_obstacle = (player.x-1, player.y);
//                     }
//                     Dir::Right => {
//                         test_map[player.y][player.x+1] = "O";
//                         new_obstacle = (player.x+1, player.y);
//                     }
//                 }

//                 // print_map(&test_map);

//                 if starting_pos != new_obstacle && // no starting pos obstacle
//                    !obstacles.contains(&new_obstacle) // different ones
//                    && run_sim_until_pos(player.clone(), test_map) { // cycles agent
//                     can_loop += 1;
//                     obstacles.push(new_obstacle);
//                 }
//                 player.move_forward();
//             }
//             Sensor::Wall => {
//                 player.rotate();
//             }
//         }
//     }
// }

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading the input");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading the input");

    let mut map: Vec<Vec<&str>> = input.split("\n").filter(|line| *line != "").map(|line| line.split("").filter(|c| *c != "").collect()).collect();

    if let Some(player) = get_starting_pos(&map) {
        let moved_places = run_sim(player.clone(), map.clone());

        let mut loop_count:i32 = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if (x,y) == (player.x,player.y) {continue;}
                if map[y][x] != "." {continue;}

                map[y][x] = "#";
                if run_sim_until_pos(player.clone(), &map) {
                    loop_count += 1;
                    println!("{loop_count}");
                }
                map[y][x] = ".";
            }
        }

        // let can_loop = run_sim_p2(player.clone(), map.clone());
        // println!("Part1: {moved_places}");
        println!("Part2: {loop_count}");
    };
}
