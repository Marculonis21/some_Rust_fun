use std::{default, fs};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

type Map<'a> = Vec<Vec<&'a str>>;

#[derive(Clone,Debug,Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Pos,
    dir: Dir
}

impl Node {
    fn expand(&self, map: &Map) -> Vec<(Node, i64)> {
        let mut next: Vec<(Node, i64)> = vec![];

        match self.dir {
            Dir::North => {
                if map[self.pos.y-1][self.pos.x] != "#" {
                    next.push((Node{
                        pos: Pos{x: self.pos.x, y: self.pos.y-1},
                        dir: self.dir.clone()}, 
                        1));
                }

                next.push((Node{pos: self.pos.clone(), dir: Dir::East}, 1000));
                next.push((Node{pos: self.pos.clone(), dir: Dir::West}, 1000));
            }
            Dir::South => {
                if map[self.pos.y+1][self.pos.x] != "#" {
                    next.push((Node{
                        pos: Pos{x: self.pos.x, y: self.pos.y+1},
                        dir: self.dir.clone()}, 
                        1));
                }

                next.push((Node{pos: self.pos.clone(), dir: Dir::East}, 1000));
                next.push((Node{pos: self.pos.clone(), dir: Dir::West}, 1000));
            }
            Dir::East => {
                if map[self.pos.y][self.pos.x+1] != "#" {
                    next.push((Node{
                        pos: Pos{x: self.pos.x+1, y: self.pos.y},
                        dir: self.dir.clone()}, 
                        1));
                }

                next.push((Node{pos: self.pos.clone(), dir: Dir::North}, 1000));
                next.push((Node{pos: self.pos.clone(), dir: Dir::South}, 1000));
            }
            Dir::West => {
                if map[self.pos.y][self.pos.x-1] != "#" {
                    next.push((Node{
                        pos: Pos{x: self.pos.x-1, y: self.pos.y},
                        dir: self.dir.clone()}, 
                        1));
                }

                next.push((Node{pos: self.pos.clone(), dir: Dir::North}, 1000));
                next.push((Node{pos: self.pos.clone(), dir: Dir::South}, 1000));
            }
        }

        return next;
    }
}

fn parse_input(input: &String) -> (Map, Pos, Pos) {
    let map: Map = input.split("\n").filter(|line| *line != "").map(|line| line.split("").filter(|c| *c != "").collect()).collect();

    let mut start = Pos::default();
    let mut end = Pos::default();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                "S" => start = Pos{x,y},
                "E" => end   = Pos{x,y},
                _ => ()
            }
        }
    }

    (map, start, end)
}

fn dijkstra(map: &Map, start: Pos, end: Pos) -> Option<(i64, Vec<Pos>)> {
    let mut pq: PriorityQueue<Node, Reverse<i64>> = PriorityQueue::new();
    let mut node_cost: HashMap<Node, i64> = HashMap::new();
    let mut path_parent: HashMap<Node, Node> = HashMap::new();

    let start_node = Node{pos: start.clone(), dir: Dir::East};
    node_cost.insert(start_node.clone(), 0);
    pq.push(start_node, Reverse(0));

    while let Some((curr_node,v)) = pq.pop() {
        if curr_node.pos == end {
            let mut path_pos:Vec<Pos> = vec![];

            // println!("at end: {curr_node:?} - {v:?}");

            let mut n = curr_node;
            let mut show_map = map.clone();

            while let Some(node) = path_parent.get(&n) {
                let s = match node.dir {
                    Dir::North => "^",
                    Dir::South => "v",
                    Dir::East  => ">",
                    Dir::West  => "<",
                };
                show_map[node.pos.y][node.pos.x] = s;
                path_pos.push(node.pos.clone());

                n = node.clone();
            }

            // print_map(&show_map);

            return Some((v.0, path_pos));
        }

        for (next_node, link_cost) in curr_node.expand(map) {

            let next_cost = node_cost[&curr_node] + link_cost;

            if !node_cost.contains_key(&next_node) {
                node_cost.insert(next_node.clone(), next_cost);
                path_parent.insert(next_node.clone(), curr_node.clone());
                pq.push(next_node, Reverse(next_cost));
            }
            else {
                let old_cost = node_cost[&next_node];

                if next_cost <= old_cost {
                    *node_cost.get_mut(&next_node).unwrap() = next_cost;
                    *path_parent.get_mut(&next_node).unwrap() = curr_node.clone();
                    pq.change_priority(&next_node, Reverse(next_cost));
                }
            }
        }
    }

    return None;
}

fn print_map(map: &Map) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}",map[y][x]);
        }
        print!("\n");
    }
}

fn main() {
    // let input = fs::read_to_string("input.txt").expect("Problem reading file");
    let input = fs::read_to_string("small_input1.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input2.txt").expect("Problem reading file");

    let (map, start, end) = parse_input(&input);

    let (best_path_len, path_pos) = dijkstra(&map, start.clone(), end.clone()).unwrap();

    let mut used_pos: HashSet<Pos> = HashSet::new();

    for p in path_pos.iter() {
        used_pos.insert(p.clone());
    }

    for p in 0..path_pos.len() {
        println!("{}/{}", p, path_pos.len());

        let mut test_map = map.clone();
        test_map[path_pos[p].y][path_pos[p].x] = "#";

        if let Some((path_len,path)) = dijkstra(&test_map, start.clone(), end.clone()) {
            if path_len == best_path_len {
                for p in path {
                    used_pos.insert(p);
                }
            }

        }
    }

    println!("{}", used_pos.len());

    // dijkstra(&map, start, end);
}
