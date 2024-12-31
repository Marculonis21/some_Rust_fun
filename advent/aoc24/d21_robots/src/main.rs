use core::panic;
use std::fs;
use std::collections::{hash_map, HashMap};

// +---+---+---+  
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Press,
}

#[derive(PartialEq)]
enum KeypadType {
    Num,
    Arrow,
}

type Keypad<'a> = HashMap<&'a str, Pos>;

fn solve_num(current: &mut Pos, end: &Pos) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    while current.x != end.x {
        if current.x > end.x {
            if current.y == 3 && 
               current.x == 1 {
                current.y -= 1;
                moves.push(Move::Up);
            }

            current.x -= 1;
            moves.push(Move::Left);
        }
        else {
            current.x += 1;
            moves.push(Move::Right);
        }
    }

    while current.y != end.y {
        if current.y > end.y {
            current.y -= 1;
            moves.push(Move::Up);
        }
        else {

            current.y += 1;
            moves.push(Move::Down);
        }
    }


    moves
}

fn solve_arrow(current: &mut Pos, end: &Pos) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    while current.y != end.y {
        if current.y > end.y {
            if current.y == 1 && 
               current.x == 0 {
               while current.x != current.y{
                   current.x += 1;
                   moves.push(Move::Right);
               }
            }

            current.y -= 1;
            moves.push(Move::Up);
        }
        else {
            current.y += 1;
            moves.push(Move::Down);
        }
    }

    while current.x != end.x {
        if current.x > end.x {
            current.x -= 1;
            moves.push(Move::Left);
        }
        else {
            current.x += 1;
            moves.push(Move::Right);
        }
    }
    
    moves
}

fn find_path(start_key: &str, end_key: &str, keypad: &Keypad, keypad_type: &KeypadType) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let start_pos: &Pos = keypad.get(start_key).unwrap();
    let end_pos: &Pos = keypad.get(end_key).unwrap();

    let mut current_pos = start_pos.clone();

    match keypad_type {
        KeypadType::Num => {
            moves = solve_num(&mut current_pos, &end_pos);
        },
        KeypadType::Arrow =>  {
            moves = solve_arrow(&mut current_pos, &end_pos);
        },
    }

    moves.push(Move::Press);

    moves
}

fn decode_moves(moves: Vec<Vec<Move>>) -> String {
    moves.iter().flatten().map(|m| {
        match m {
            Move::Up => "^",
            Move::Down => "v",
            Move::Left => "<",
            Move::Right => ">",
            Move::Press => "A",
        }
    }).collect::<Vec<&str>>().join("")
}

fn button_procedure(code: &str, keypad: &Keypad, keypad_type: &KeypadType) -> String {
    let code = code.split("").filter(|c| *c != "").collect::<Vec<&str>>();
    // println!("code: {:?}", code);

    let mut moves_all: Vec<Vec<Move>> = vec![];

    let mut start_key = "A";

    for c in code {
        assert!(keypad.contains_key(c), "we are probably using wrong keypad!");

        let moves = find_path(start_key, c, keypad, keypad_type);
        // println!("{start_key} -> {c}  =>  {moves:?}");
        moves_all.push(moves);

        start_key = c;
    }

    decode_moves(moves_all)
}

fn main() {
    // let input = fs::read_to_string("input.txt").expect("Problem reading file");
    let input = fs::read_to_string("small_input.txt").expect("Problem reading file");

    let mut number_keypad: Keypad = HashMap::new();
    number_keypad.insert("7", Pos{x:0,y:0});
    number_keypad.insert("8", Pos{x:1,y:0});
    number_keypad.insert("9", Pos{x:2,y:0});
    number_keypad.insert("4", Pos{x:0,y:1});
    number_keypad.insert("5", Pos{x:1,y:1});
    number_keypad.insert("6", Pos{x:2,y:1});
    number_keypad.insert("1", Pos{x:0,y:2});
    number_keypad.insert("2", Pos{x:1,y:2});
    number_keypad.insert("3", Pos{x:2,y:2});
    number_keypad.insert("0", Pos{x:1,y:3});
    number_keypad.insert("A", Pos{x:2,y:3});

    let mut arrow_keypad: Keypad = HashMap::new();
    arrow_keypad.insert("^", Pos{x:1,y:0});
    arrow_keypad.insert("A", Pos{x:2,y:0});
    arrow_keypad.insert("<", Pos{x:0,y:1});
    arrow_keypad.insert("v", Pos{x:1,y:1});
    arrow_keypad.insert(">", Pos{x:2,y:1});

    let codes: Vec<&str> = input.split("\n").filter(|line| *line != "").collect();
    for c in codes {
        let k1_moves = button_procedure(&c, &number_keypad, &KeypadType::Num);
        let k2_moves = button_procedure(&k1_moves, &arrow_keypad, &KeypadType::Arrow);
        println!("{k2_moves}");
        break;
        let k3_moves = button_procedure(&k2_moves, &arrow_keypad, &KeypadType::Arrow);
        println!("{c}: {k3_moves}");
    }
}
