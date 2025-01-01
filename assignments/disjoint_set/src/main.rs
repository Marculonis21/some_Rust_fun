use std::io;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(count: usize) -> Self { 
        let mut parent: Vec<usize> = vec![];
        let mut size: Vec<usize> = vec![];
        parent.resize(count,0);
        size.resize(count,0);

        for i in 0..count {
            parent[i] = i ;
            size[i] = 1;
        }

        return Self { parent: parent,
               size: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;

        while self.parent[root] != root {
            self.parent[root] = self.parent[self.parent[root]];
            root = self.parent[root];
        }

        return root
    }

    fn merge(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);

        if self.size[ry] > self.size[rx] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        }
        else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut uf = UnionFind::new(0);

    for line in stdin.lines() {
        let line = line.expect("Could not read line").trim().to_string();
        let parts:Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["init", n] => {
                // println!(">> init {n}");
                uf = UnionFind::new(n.parse().unwrap());
            }

            ["merge", a, b] => {
                // println!(">> merge {a} {b}");
                uf.merge(a.parse().unwrap(),
                         b.parse().unwrap());   
            }
            ["find", x] => {
                // println!(">> find {x}");
                let out = uf.find(x.parse().unwrap());
                println!("{out}");
            }
            _ => panic!("Unknown input!")
        }
    }
}
