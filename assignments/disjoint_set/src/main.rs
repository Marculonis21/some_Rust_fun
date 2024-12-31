use std::io;

struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
    elements: i32
}

impl UnionFind {
    fn new(count: usize) -> Self { 
        let mut parent: Vec<i32> = vec![];
        let mut size: Vec<i32> = vec![];
        parent.resize(count,0);
        size.resize(count,0);

        for i in 0..count {
            parent[i] = i as i32;
            size[i] = 1;
        }

        return Self { parent: parent,
               size: size,
               elements: count as i32
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        let mut root = x;

        while self.parent[root as usize] != root {
            self.parent[root as usize] = self.parent[self.parent[root as usize] as usize];
            root = self.parent[root as usize];
        }

        return root
    }

    fn merge(&mut self, x: i32, y: i32) {
        let rx = self.find(x);
        let ry = self.find(y);

        if self.size[ry as usize] > self.size[rx as usize] {
            self.parent[rx as usize] = ry;
            self.size[ry as usize] += self.size[rx as usize];
        }
        else {
            self.parent[ry as usize] = rx;
            self.size[rx as usize] += self.size[ry as usize];
        }

    }
}

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();

    let mut uf = UnionFind::new(0);

    for line in stdin.lines() {
        let line = line.expect("Could not read line").trim().to_string();
        let parts:Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["init", n] => {
                println!(">> init {n}");
                uf = UnionFind::new(n.parse().unwrap());
            }

            ["merge", a, b] => {
                println!(">> merge {a} {b}");
                uf.merge(a.parse().unwrap(),
                         b.parse().unwrap());   
            }
            ["find", x] => {
                println!(">> find {x}");
                let out = uf.find(x.parse().unwrap());
                println!("OUT: {out}");
            }
            _ => panic!("Unknown input!")
        }
    }
}
