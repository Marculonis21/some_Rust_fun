use std::{fs, io::Read, str::FromStr};
use regex::Regex;
use std::io;

#[derive(Default, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn modulo(&mut self, size_x: i32, size_y: i32) {
        self.x = self.x.rem_euclid(size_x);
        self.y = self.y.rem_euclid(size_y);
    }

    fn apply(&mut self, vel: &Vel) {
        self.x += vel.x;
        self.y += vel.y;
    }
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        Self {x, y}
    }
}


#[derive(Default, Debug)]
struct Vel {
    x: i32,
    y: i32,
}

impl From<&str> for Vel {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        Self {x, y}
    }
}

#[derive(Default, Debug)]
struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn new(pos: &str, vel: &str) -> Self {
        Self { pos: Pos::from(pos), vel: Vel::from(vel) }
    }

    fn sim(&mut self, time: usize, size_x: i32, size_y: i32) {
        for _ in 0..time {
            self.pos.apply(&self.vel);
            self.pos.modulo(size_x, size_y);
        }
    }
}

fn should_pause(map: &Vec<Vec<i32>>) -> bool {
    for y in 0..map.len()-2 {
        for x in 0..map[0].len()-2 {
            if map[y][x] > 0 && map[y][x+1] > 0 && map[y][x+1] > 0 {
                if map[y+1][x] > 0 && map[y+1][x+1] > 0 && map[y+1][x+1] > 0 {
                    if map[y+2][x] > 0 && map[y+2][x+1] > 0 && map[y+2][x+1] > 0 {
                        return true
                    }
                }
            }
        }
    }

    false
}

fn draw_map(map: &Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                print!(".");
            }
            else {
                print!("{}", map[y][x]);
            }
        }
        print!("\n");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    let size_x: i32 = 101;
    let size_y: i32 = 103;

    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    // let size_x: i32 = 11;
    // let size_y: i32 = 7;

    let re = Regex::new(r"p=(-?\d+,-?\d+) v=(-?\d+,-?\d+)").unwrap();

    let mut robots: Vec<Robot> = re.captures_iter(&input).map(|capture| {
        Robot::new(&capture[1], &capture[2])
    }).collect();

    // let mut map: Vec<Vec<i32>> = vec![vec![0; size_x as usize]; size_y as usize];
    // P1
    // robots.into_iter().for_each(|mut r| {
    //     r.sim(100, size_x, size_y);
    //     map[r.pos.y as usize][r.pos.x as usize] += 1;
    // });
    
    // P2
    let stdin = io::stdin();
    let mut pause = false;
    let mut i = 1;
    loop {
        let mut map: Vec<Vec<i32>> = vec![vec![0; size_x as usize]; size_y as usize];
        for r in robots.iter_mut() {
            r.sim(1, size_x, size_y);
            map[r.pos.y as usize][r.pos.x as usize] += 1;
        }
        pause = should_pause(&map);

        if pause {
            draw_map(&map);

            println!("It's timestep: {i}");
            let mut s: String = "".into();
            let _ = stdin.read_line(&mut s);
        }
        i+=1;
    }

//     let lu = (0..3).map(|y| (0..5 ).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let ru = (0..3).map(|y| (6..11).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let ld = (4..7).map(|y| (0..5 ).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let rd = (4..7).map(|y| (6..11).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();

//     let lu = (0..51  ).map(|y| (0..50  ).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let ru = (52..103).map(|y| (0..50  ).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let ld = (0..51  ).map(|y| (51..101).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();
//     let rd = (52..103).map(|y| (51..101).map(|x| map[y][x]).sum::<i32>()).sum::<i32>();

    // println!("{}", lu*ru*ld*rd);

}
