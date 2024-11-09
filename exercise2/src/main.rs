use std::io::stdin;

fn quicksort(v: Vec<u64>) -> Vec<u64> {
    if v.len() <= 1 {
        return v;
    }

    let pivot = v[0];
    let mut left = vec![];
    let mut right = vec![];

    for item in v {
        if item < pivot {
            left.push(item);
        } else if item > pivot {
            right.push(item);
        }
    }

    let mut result = vec![];
    result.extend_from_slice(&quicksort(left));
    result.push(pivot);
    result.extend_from_slice(&quicksort(right));
    return result;
}

#[derive(Debug)]
struct BinaryTree {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    value: u64,
}

impl BinaryTree {
    fn new(x: u64) -> Self {
        return Self {
            left: None,
            right: None,
            value: x,
        };
    }

    fn push(&mut self, x: u64) {
        // WOMEN::
        
        if x < self.value {
            if let Some(l) = &mut self.left {
                l.push(x);
            }
            else {
                self.left = Some(Box::new(BinaryTree::new(x)));
            }
        }
        else {
            if let Some(r) = &mut self.right {
                r.push(x);
            }
            else {
                self.right = Some(Box::new(BinaryTree::new(x)));
            }
        }
    }

    fn contains(&self, x: u64) -> bool {
        if x < self.value {
            if let Some(l) = &self.left {
                return l.contains(x)
            } else {
                return false
            }
        }
        else if x > self.value {
            if let Some(r) = &self.right {
                return r.contains(x)
            }
            else {
                return false
            }
        }

        return true
    }
}

fn test(x: u64, f: fn(u64) -> u64) {
    println!("{}", f(x))
}

fn add_one(x: u64) -> u64{
    x + 1
}

fn main() {
    //// QUICKSORT
    // let stdin = stdin();
    // let mut nums = String::new();
    // let _ = stdin.read_line(&mut nums);

    // let nums = nums.split_whitespace().collect::<Vec<_>>();
    // let parsed_numbers :Vec<u64> = nums.clone()
    //     .into_iter()
    //     .map(|item| item.parse().unwrap())
    //     .collect();

    // let sorted = quicksort(parsed_numbers.clone());
    // println!("From {:?} to {:?}", nums, sorted);
    
    //// EX2. BINARY SEARCH TREE
    // let mut bt = BinaryTree::new(10);
    // bt.push(5);
    // bt.push(12);
    // bt.push(9);
    // bt.push(15);

    // // pretty print
    // println!("{:#?}", bt);
    
    //// function types/ traits
    // sometimes this needs to be done because test, can consume the function 
    // from the global space, which is fun and weird
    test(3, add_one as fn(u64) -> u64);
}
