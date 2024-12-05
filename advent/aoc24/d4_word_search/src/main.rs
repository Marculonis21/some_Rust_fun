use std::fs;

#[derive(Debug)]
enum Conditions {
    Horizontal(usize),
    RHorizontal(usize),
    Vertical(usize),
    RVertical(usize),

    DiagRightD(usize, usize),
    DiagRightU(usize, usize),
    DiagLeftD(usize, usize),
    DiagLeftU(usize, usize),
}

fn get_direction(char_matrix: &Vec<Vec<&str>>, cond: Conditions, cx:usize, cy:usize, max_x:usize, max_y:usize) -> Option<String> {
    // println!("{cond:#?} {cx} {cy} {max_x} {max_y}");
    match cond {
        Conditions::Horizontal(x) if cx+x < max_x                   => Some(vec![char_matrix[cy][cx], char_matrix[cy][cx+1], char_matrix[cy][cx+2], char_matrix[cy][cx+3]].concat()),
        Conditions::Vertical(y) if cy+y < max_y                     => Some(vec![char_matrix[cy][cx], char_matrix[cy+1][cx], char_matrix[cy+2][cx], char_matrix[cy+3][cx]].concat()),
        Conditions::RHorizontal(x) if (cx as i32) - (x as i32) >= 0 => Some(vec![char_matrix[cy][cx], char_matrix[cy][cx-1], char_matrix[cy][cx-2], char_matrix[cy][cx-3]].concat()),
        Conditions::RVertical(y) if (cy as i32) - (y as i32) >= 0   => Some(vec![char_matrix[cy][cx], char_matrix[cy-1][cx], char_matrix[cy-2][cx], char_matrix[cy-3][cx]].concat()),

        Conditions::DiagRightD(x, y) if cx+x < max_x && cy+y < max_y                                   => Some(vec![char_matrix[cy][cx], char_matrix[cy+1][cx+1], char_matrix[cy+2][cx+2], char_matrix[cy+3][cx+3]].concat()),
        Conditions::DiagRightU(x, y) if cx+x < max_x && (cy as i32) - (y as i32) >= 0                  => Some(vec![char_matrix[cy][cx], char_matrix[cy-1][cx+1], char_matrix[cy-2][cx+2], char_matrix[cy-3][cx+3]].concat()),
        Conditions::DiagLeftD (x, y) if (cx as i32) - (x as i32) >= 0 && cy+y < max_y                  => Some(vec![char_matrix[cy][cx], char_matrix[cy+1][cx-1], char_matrix[cy+2][cx-2], char_matrix[cy+3][cx-3]].concat()),
        Conditions::DiagLeftU (x, y) if (cx as i32) - (x as i32) >= 0 && (cy as i32) - (y as i32) >= 0 => Some(vec![char_matrix[cy][cx], char_matrix[cy-1][cx-1], char_matrix[cy-2][cx-2], char_matrix[cy-3][cx-3]].concat()),
        _ => None
    }
}

fn get_direction_p2(char_matrix: &Vec<Vec<&str>>, cond: Conditions, cx:usize, cy:usize, max_x:usize, max_y:usize) -> Option<String> {
    if !(cx+1 < max_x && (cx as i32) - (1 as i32) >= 0 &&
         cy+1 < max_y && (cy as i32) - (1 as i32) >= 0)
    {
        return None
    }

    match cond {
        Conditions::DiagRightD(_,_) => Some(vec![char_matrix[cy-1][cx-1], char_matrix[cy][cx], char_matrix[cy+1][cx+1]].concat()),
        Conditions::DiagRightU(_,_) => Some(vec![char_matrix[cy+1][cx-1], char_matrix[cy][cx], char_matrix[cy-1][cx+1]].concat()),
        Conditions::DiagLeftD (_,_) => Some(vec![char_matrix[cy-1][cx+1], char_matrix[cy][cx], char_matrix[cy+1][cx-1]].concat()),
        Conditions::DiagLeftU (_,_) => Some(vec![char_matrix[cy+1][cx+1], char_matrix[cy][cx], char_matrix[cy-1][cx-1]].concat()),
        _ => None
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input Problem");

    let mut char_matrix: Vec<Vec<&str>> = input.split("\n").map(|line| line.split("").collect::<Vec<&str>>()).collect();
    char_matrix.pop();

    let max_x = char_matrix[0].len();
    let max_y = char_matrix.len();

    let mut cp1 = 0;
    let mut cp2 = 0;
    for cy in 0..max_y {
        for cx in 0..max_x {
            match get_direction(&char_matrix, Conditions::Horizontal(3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::Vertical(3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::RHorizontal(3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::RVertical(3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::DiagRightD(3,3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::DiagRightU(3,3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::DiagLeftD(3,3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }
            match get_direction(&char_matrix, Conditions::DiagLeftU(3,3), cx, cy, max_x, max_y) {
                Some(text) if text == "XMAS" => cp1 += 1,
                _ => ()
            }

            let mut p2_inner = 0;
            match get_direction_p2(&char_matrix, Conditions::DiagRightU(1,1), cx, cy, max_x, max_y) {
                Some(text) if text == "MAS" => p2_inner += 1,
                _ => ()
            }
            match get_direction_p2(&char_matrix, Conditions::DiagRightD(1,1), cx, cy, max_x, max_y) {
                Some(text) if text == "MAS" => p2_inner += 1,
                _ => ()
            }
            match get_direction_p2(&char_matrix, Conditions::DiagLeftU(1,1), cx, cy, max_x, max_y) {
                Some(text) if text == "MAS" => p2_inner += 1,
                _ => ()
            }
            match get_direction_p2(&char_matrix, Conditions::DiagLeftD(1,1), cx, cy, max_x, max_y) {
                Some(text) if text == "MAS" => p2_inner += 1,
                _ => ()
            }

            if p2_inner > 1 {
                cp2 += 1;
            }
        }
    }

    println!("Part1: {}",cp1);
    println!("Part2: {}",cp2);
}
