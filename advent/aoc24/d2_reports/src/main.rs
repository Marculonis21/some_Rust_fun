use std::{fs, slice};


fn get_reports(input: &String) -> Vec<Vec<i32>> {
    input.split("\n").filter(|line| *line != "")    
                     .map(|line| line.split_whitespace()
                                     .map(|x| x.parse().unwrap())
                                     .collect())
                     .collect()
}

fn check_safety(line: &Vec<i32>) -> bool {
    if line[0] == line[1] { return false }

    let increase = line[0] < line[1];
    line.windows(2).map(|win| {
        let (a,b) = (win[0],win[1]);

        let diff = i32::abs(a-b);

        ((increase && a < b) || (!increase && a > b)) && (diff < 4)
    }).fold(true, |acc, x| acc && x)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input Problem");
    let count = get_reports(&input).iter().filter(|line| check_safety(line)).count();
    println!("Part1: {}",count);
    let count = get_reports(&input).iter().filter(|line| {
        let mut line_proxy;
        let mut cond = false;
        // really inneficient
        for i in 0..line.len() {
            line_proxy = line.to_vec();
            line_proxy.remove(i);
            cond = cond || check_safety(&line_proxy);
        }
        return cond;
    }).count();
    println!("Part2: {}",count);
}
