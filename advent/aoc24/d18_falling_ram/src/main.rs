use std::fs;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type Map = Vec<Vec<bool>>;

fn sim_falling(input: &str, sim_steps: usize, map_dimensions: usize) -> Map {
    let mut map: Map = vec![vec![true; map_dimensions]; map_dimensions];

    input.split("\n").take(sim_steps).for_each(|line| {
        let (x, y): (usize, usize) = line
            .split_once(',')
            .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
            .unwrap();
        map[y][x] = false;
    });

    return map;
}

fn print_map(map: &Map) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                print!(".");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn expand(&self, map: &Map) -> Vec<(Self, i64)> {
        let mut next: Vec<(Pos, i64)> = vec![];

        if self.x < map[0].len()-1 && map[self.y][self.x+1] {
            next.push((Pos{x:self.x+1, y:self.y}, 1));
        }
        if self.x > 0 && map[self.y][self.x-1] {
            next.push((Pos{x:self.x-1, y:self.y}, 1));
        }
        if self.y < map.len()-1 && map[self.y+1][self.x] {
            next.push((Pos{x:self.x, y:self.y+1}, 1));
        }
        if self.y > 0 && map[self.y-1][self.x] {
            next.push((Pos{x:self.x, y:self.y-1}, 1));
        }

        return next;
    }
}

fn dijkstra(map: &Map, start: &Pos, end: &Pos) -> Option<i64> {
    let mut pq: PriorityQueue<Pos, Reverse<i64>> = PriorityQueue::new();
    let mut cost: HashMap<Pos, i64> = HashMap::new();
    let mut path_parent: HashMap<Pos, Pos> = HashMap::new();

    let start_pos = start.clone();

    cost.insert(start_pos.clone(), 0);
    pq.push(start_pos, Reverse(0));

    while let Some((curr_pos, v)) = pq.pop() {
        if curr_pos == *end {
            // let mut path_pos: Vec<Pos> = vec![];

            // // println!("at end: {curr_node:?} - {v:?}");

            // let mut n = curr_pos;

            // while let Some(pos) = path_parent.get(&n) {
            //     // show_map[pos.y][pos.x] = true;
            //     path_pos.push(pos.clone());
            // }

            // // print_map(&show_map);

            // return Some((v.0, path_pos));
            return Some(v.0);
        }

        for (next_pos, link_cost) in curr_pos.expand(map) {
            let next_cost = cost[&curr_pos] + link_cost;

            if !cost.contains_key(&next_pos) {
                cost.insert(next_pos.clone(), next_cost);
                path_parent.insert(next_pos.clone(), curr_pos.clone());
                pq.push(next_pos, Reverse(next_cost));
            } else {
                let old_cost = cost[&next_pos];

                if next_cost <= old_cost {
                    *cost.get_mut(&next_pos).unwrap() = next_cost;
                    *path_parent.get_mut(&next_pos).unwrap() = curr_pos.clone();
                    pq.change_priority(&next_pos, Reverse(next_cost));
                }
            }
        }
    }

    return None;
}

fn bin_search(input: &str, low: usize, high:usize, map_dimensions: usize, start: &Pos, end: &Pos) -> usize {
    if high < low {
        return high;
    }

    let middle = dbg!(low + (high-low)/2);
    let map = sim_falling(input, middle, map_dimensions);

    if let None = dijkstra(&map, start, end) {
        return bin_search(input, low, middle-1, map_dimensions, start, end);
    }
    else {
        return bin_search(input, middle+1, high, map_dimensions, start, end);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    let map = sim_falling(&input, 1024, 71);
    let (start, end) = (Pos{x:0,y:0}, Pos{x:70,y:70});
    //
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    // let map = sim_falling(&input, 12, 7);
    // let (start, end) = (Pos{x:0,y:0}, Pos{x:6,y:6});

    // let map = sim_falling(&input, 1024, 71);
    print_map(&map);

    let input_len = input.split("\n").count() - 1;

    let out = dijkstra(&map, &start, &end);
    println!("Part1: {}", out.unwrap());

    let bin_search_output = bin_search(&input, 0, input_len, 71, &start, &end);

    let test_map = sim_falling(&input, bin_search_output+1, 71);
    let out = dijkstra(&test_map, &start, &end);
    println!("Test after bin_search: id:{} {:?} (should be none)", bin_search_output, out);

    println!("Part2: {:?},", input.split("\n").nth(bin_search_output).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_input() {
        let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
        let map = sim_falling(&input, 12, 7);
        let out = dijkstra(&map, &Pos{x:0,y:0}, &Pos{x:6,y:6}).unwrap();

        assert_eq!(out, 22)
    }
}
