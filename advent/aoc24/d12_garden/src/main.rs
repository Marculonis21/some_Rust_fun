use std::{collections::VecDeque, f32::MAX_EXP, fs, usize};


#[derive(PartialEq, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn get_surround(&self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let mut surround: Vec<Pos> = vec![];

        if self.x+1 < max_x {
            surround.push(Pos{x:self.x+1, y:self.y})
        }
        if self.x > 0 {
            surround.push(Pos{x:self.x-1, y:self.y})
        }

        if self.y+1 < max_y {
            surround.push(Pos{x:self.x, y:self.y+1})
        }
        if self.y > 0 {
            surround.push(Pos{x:self.x, y:self.y-1})
        }

        return surround
    }

    fn get_surround_option(&self, max_x: usize, max_y: usize) -> Vec<Option<Pos>> {
        let mut surround: Vec<Option<Pos>> = vec![];

        if self.y > 0 {
            surround.push(Some(Pos{x:self.x, y:self.y-1}))
        }
        else {
            surround.push(None)
        }

        if self.y+1 < max_y {
            surround.push(Some(Pos{x:self.x, y:self.y+1}))
        }
        else {
            surround.push(None)
        }

        if self.x > 0 {
            surround.push(Some(Pos{x:self.x-1, y:self.y}))
        }
        else {
            surround.push(None)
        }

        if self.x+1 < max_x {
            surround.push(Some(Pos{x:self.x+1, y:self.y}))
        }
        else {
            surround.push(None)
        }


        return surround
    }
}

#[derive(Default, Debug)]
struct Region {
    reg_type: String,
    parts: Vec<Pos>,
}

impl Region {
    fn contains(&self, pos: &Pos) -> bool {
        for other in self.parts.iter() {
            if other == pos {
                return true;
            }
        }

        return false;
    }

    fn get_region_from_map(map: &Vec<Vec<&str>>, pos: &Pos, used_map: &mut Vec<Vec<bool>>) -> Option<Self> {
        if used_map[pos.y][pos.x] { return None; }

        let mut new_reg = Region::default();
        new_reg.reg_type = map[pos.y][pos.x].to_string();

        // bfs fill
        let mut queue: VecDeque<Pos> = VecDeque::new();
        queue.push_back(pos.clone());

        while let Some(curr) = queue.pop_front() {
            if map[curr.y][curr.x] != new_reg.reg_type { continue; }

            new_reg.parts.push(curr.clone());
            used_map[curr.y][curr.x] = true;

            for next in curr.get_surround(map[0].len(), map.len()) {
                if queue.contains(&next) || used_map[next.y][next.x] { continue; }

                queue.push_back(next);
            }
        }

        return Some(new_reg)
    }

    fn area(&self) -> usize {
        self.parts.len()
    }

    fn perimeter(&self, map_max_x: usize, map_max_y: usize) -> usize {
        let mut borders = 0;
        for p in self.parts.iter() {
            let surround = p.get_surround(map_max_x, map_max_y);

            // get border for each edge map position
            borders += 4 - surround.len();

            for other in surround.iter() {
                if !self.contains(other) { 
                    borders += 1; 
                }
            }
        }

        borders
    }

    fn perimeter_p2(&self, map_max_x: usize, map_max_y: usize) -> usize {
        let mut missing_n: Vec<Pos> = vec![];
        let mut missing_s: Vec<Pos> = vec![];
        let mut missing_e: Vec<Pos> = vec![];
        let mut missing_w: Vec<Pos> = vec![];

        for p in self.parts.iter() {
            let surround = p.get_surround_option(map_max_x, map_max_y);

            let (n,s,e,w) = (surround[0].clone(),surround[1].clone(),surround[2].clone(),surround[3].clone());

            // either is missing or has neigbhor of different type
            match n {
                None => missing_n.push(p.clone()),
                Some(pos) if !self.contains(&pos) => missing_n.push(p.clone()),
                _ => (),
            }
            match s {
                None => missing_s.push(p.clone()),
                Some(pos) if !self.contains(&pos) => missing_s.push(p.clone()),
                _ => (),
            }
            match e {
                None => missing_e.push(p.clone()),
                Some(pos) if !self.contains(&pos) => missing_e.push(p.clone()),
                _ => (),
            }
            match w {
                None => missing_w.push(p.clone()),
                Some(pos) if !self.contains(&pos) => missing_w.push(p.clone()),
                _ => (),
            }
        }

        let mut n_border = 0;
        // missing top
        while let Some(m) = missing_n.pop() {
            n_border += 1;
            
            // left/right check + remove found items
            for i in 1..map_max_x {
                if m.x-(i-1) == 0 { break; }

                let check = Pos{x: m.x-i, y: m.y};
                // if contains -> remove such item
                if missing_n.contains(&check) { missing_n.retain(|x| *x != check); }
                else { break; }
            }

            for i in 1..map_max_x {
                if m.x+(i-1) == map_max_x-1 { break; }

                let check = Pos{x: m.x+i, y: m.y};
                // if contains -> remove such item
                if missing_n.contains(&check) { missing_n.retain(|x| *x != check); }
                else { break; }
            }
        }

        let mut s_border = 0;
        // missing top
        while let Some(m) = missing_s.pop() {
            s_border += 1;
            
            // left/right check + remove found items
            for i in 1..map_max_x {
                if m.x-(i-1) == 0 { break; }

                let check = Pos{x: m.x-i, y: m.y};
                // if contains -> remove such item
                if missing_s.contains(&check) { missing_s.retain(|x| *x != check); }
                else { break; }
            }

            for i in 1..map_max_x {
                if m.x+(i-1) == map_max_x-1 { break; }

                let check = Pos{x: m.x+i, y: m.y};
                // if contains -> remove such item
                if missing_s.contains(&check) { missing_s.retain(|x| *x != check); }
                else { break; }
            }
        }


        let mut e_border = 0;
        // missing top
        while let Some(m) = missing_e.pop() {
            e_border += 1;
            
            // left/right check + remove found items
            for i in 1..map_max_y {
                if m.y-(i-1) == 0 { break; }

                let check = Pos{x: m.x, y: m.y-i};
                // if contains -> remove such item
                if missing_e.contains(&check) { missing_e.retain(|x| *x != check); }
                else { break; }
            }

            for i in 1..map_max_y {
                if m.y+(i-1) == map_max_y-1 { break; }

                let check = Pos{x: m.x, y: m.y+i};
                // if contains -> remove such item
                if missing_e.contains(&check) { missing_e.retain(|x| *x != check); }
                else { break; }
            }
        }

        let mut w_border = 0;
        // missing top
        while let Some(m) = missing_w.pop() {
            w_border += 1;
            
            // left/right check + remove found items
            for i in 1..map_max_y {
                if m.y-(i-1) == 0 { break; }

                let check = Pos{x: m.x, y: m.y-i};
                // if contains -> remove such item
                if missing_w.contains(&check) { missing_w.retain(|x| *x != check); }
                else { break; }
            }

            for i in 1..map_max_y {
                if m.y+(i-1) == map_max_y-1 { break; }

                let check = Pos{x: m.x, y: m.y+i};
                // if contains -> remove such item
                if missing_w.contains(&check) { missing_w.retain(|x| *x != check); }
                else { break; }
            }
        }

        return n_border + s_border + e_border + w_border;
    }
}

fn process_str(input: String) -> usize {
    let map: Vec<Vec<&str>> = input
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| line.split("").filter(|c| *c != "").collect())
        .collect();

    let mut used_map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut regions: Vec<Region> = vec![];

    let mut cost_sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(reg) = Region::get_region_from_map(&map, &Pos{x,y}, &mut used_map) {

                // ----------------------------------> P2 change
                cost_sum += reg.area() * reg.perimeter_p2(map[0].len(), map.len());
                regions.push(reg);
            }
        }
    }

    return cost_sum;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let input = fs::read_to_string("small_input.txt").expect("Problem reading file");

    let map: Vec<Vec<&str>> = input
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| line.split("").filter(|c| *c != "").collect())
        .collect();

    let mut used_map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut regions: Vec<Region> = vec![];

    let mut cost_sum_p1 = 0;
    let mut cost_sum_p2 = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(reg) = Region::get_region_from_map(&map, &Pos{x,y}, &mut used_map) {
                cost_sum_p1 += reg.area() *reg.perimeter(map[0].len(), map.len());
                cost_sum_p2 += reg.area() *reg.perimeter_p2(map[0].len(), map.len());
                regions.push(reg);
            }
        }
    }

    println!("Part1: {}", cost_sum_p1);
    println!("Part2: {}", cost_sum_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let input = "AAAA
BBCD
BBCC
EEEC".into();

        assert_eq!(process_str(input), 80);
    }

    #[test]
    fn xo_test() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO".into();

        assert_eq!(process_str(input), 436);
    }

    #[test]
    fn e_shaped_test() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE".into();

        assert_eq!(process_str(input), 236);
    }

    #[test]
    fn AB_test() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA".into();

        assert_eq!(process_str(input), 368);
    }

    #[test]
    fn BIG_test() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE".into();

        assert_eq!(process_str(input), 1206);
    }
}
