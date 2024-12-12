use std::fs;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Stone {
    value: u64
}

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        Self{ value: value.parse().expect("Should be parsing numbers") }
    }
}

impl Stone {
    fn apply_rule(&self) -> Vec<Stone> {
        let mut out = vec![];
        if self.value == 0 {
            out.push(Stone{value: 1});
        }
        else if self.value.to_string().len() % 2 == 0 {
            let s = self.value.to_string();
            let (a,b) = s.split_at(s.len()/2);
            out.push(Stone{value: a.parse().unwrap()});
            out.push(Stone{value: b.parse().unwrap()});
        }
        else {
            out.push(Stone{value: self.value * 2024});
        }

        out
    }
}

fn one_iter(stones: Vec<Stone>) -> Vec<Stone> {
    let mut new_stones: Vec<Stone> = vec![];
    for s in stones {
        let more_stones = s.apply_rule();
        for m in more_stones {
            new_stones.push(m);
        }
    }

    new_stones
}

fn dfs_iter(stone: &Stone, depth: i8, max_depth:i8) -> i32 {
    if depth == max_depth {
        return 1;
    }

    let mut count = 0;
    let more_stones = stone.apply_rule();

    for next in more_stones {
        count += dfs_iter(&next, depth+1, max_depth);
    }

    return count;
}

fn dfs_iter_with_memory(memory: &mut Vec<HashMap<Stone, u64>>, stone: Stone, depth: usize, max_depth: usize) -> u64 {
    if depth == max_depth {
        return 1;
    }

    if memory[depth].contains_key(&stone) {
        return memory[depth][&stone];
    }

    let mut count = 0;
    for next in stone.apply_rule() {
        count += dfs_iter_with_memory(memory, next.clone(), depth+1, max_depth);
    }

    memory[depth].insert(stone, count);

    return count;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    
    let mut stones_p1: Vec<Stone> = input.split_whitespace().map(|part| Stone::from(part)).collect();
    let stones_p2: Vec<Stone> = input.split_whitespace().map(|part| Stone::from(part)).collect();

    for _ in 0..25 {
        stones_p1 = one_iter(stones_p1);
    }

    let mut memory: Vec<HashMap<Stone, u64>> = vec![HashMap::new(); 75];
    let mut res = 0;
    for s in stones_p2 {
        res += dfs_iter_with_memory(&mut memory, s.clone(), 0, 75);
    }

    // for i in 0..75 {
    //     println!("{} - {}", i, memory[i].len());
    // }

    println!("Part1 {:?}", stones_p1.len());
    println!("Part2 {}",res);
}
