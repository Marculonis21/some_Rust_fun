use core::panic;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem opening the file");
    let max_red : u64 = 12;
    let max_green : u64= 13;
    let max_blue  : u64= 14;

    let ok_games_id_sum = input.split("\n")
                                    .filter(|line| *line != "")
                                    .map(|line| {
                                        let parts = line.split_once(":").unwrap();
                                        let game_num = parts.0.split_whitespace().last().unwrap();
                                        let shows : Vec<_> = parts.1.split(";").collect();
                                        (game_num.parse::<u64>().unwrap(), shows)
                                    })
                                    .filter(|(_, shows)| {
                                        for s in shows {
                                            let test_pairs : Vec<(&str, &str)> = s.split(",").map(|item| item.trim().split_once(" ").unwrap()).collect();
                                            for (s_amount, color) in test_pairs {
                                                let amount : u64 = s_amount.parse().unwrap();
                                                match color {
                                                    "red" => if max_red < amount { return false },
                                                    "green" => if max_green < amount { return false },
                                                    "blue" => if max_blue < amount { return false },
                                                    _ => panic!("There is another color???")
                                                }
                                            }
                                        }
                                        true
                                    }).fold(0, |acc, x| acc+x.0);

    println!("{:#?}",ok_games_id_sum);
}

