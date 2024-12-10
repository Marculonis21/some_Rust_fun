use core::panic;
use std::{char::from_u32_unchecked, fs::{self}, usize};

#[derive(Clone, Debug)]
enum DS {
    File(i32, i32),
    Space(i32)
}

fn copy_old(nums: &Vec<usize>, new_space: &mut Vec<i32>) -> Vec<DS> {
    let mut id: i32 = 0;
    let mut pointer: usize = 0;
    let mut file_flag = true;
    let mut vds: Vec<DS> = vec![];

    nums.iter().for_each(|&n| {
        if !file_flag {
            file_flag = true;
            pointer += n;

            if n != 0{
                vds.push(DS::Space(n as i32));
            }
            return;
        }

        for _ in 0..n {
            file_flag = false;

            new_space[pointer] = id;
            pointer += 1;
        }
        if n != 0 {
            vds.push(DS::File(id, n as i32));
        }
        id += 1;
    });

    return vds;
}

fn compress(disk: &mut Vec<i32>) {
    let mut lp: usize = 0;
    let mut rp: usize = disk.len()-1;

    while lp < rp {
        if disk[rp] == -1 {
            rp -= 1;
            continue;
        }
        if disk[lp] != -1 {
            lp += 1;
            continue;
        }

        (disk[lp], disk[rp]) = (disk[rp], -1);
        lp += 1;
        rp -= 1;
    }

    println!("Compressed");
}

fn find_best_switch_pos(disk: &mut Vec<DS>, lp: usize, free_space_size: i32) -> Option<(usize,i32)> {
    let mut test = disk.len()-1;
    let mut best_size_diff: i32 = i32::MAX;
    let mut best_pos: Option<(usize,i32)> = None;

    while lp < test {
        match disk[test] {
            DS::File(_, file_size) => {
                // check if it fits the gap better than others before
                if file_size <= free_space_size {
                    let diff = i32::abs(free_space_size - file_size);
                    // if diff < best_size_diff {
                    //     best_pos = Some((test, diff));
                    //     best_size_diff = diff;
                    // }
                    
                    return Some((test,diff));
                }
            },
            DS::Space(_) => (),
        }

        test -= 1;
    }

    // return best_pos;
    return None
}

fn compress_ds(disk: &mut Vec<DS>) {
    let mut lp: usize = 0;
    let mut free_space_size: i32;

    loop {
        if !(lp < disk.len()) { break; }
        if let DS::File(_,_) = disk[lp] { lp += 1; continue; }

        // lp always space ... from here

        free_space_size = match disk[lp] {
            DS::Space(size) => size as i32,
            _ => panic!("This should never happen")
        };

        // loop to find the best switch position
        if let Some((sp, diff)) = find_best_switch_pos(disk, lp, free_space_size) {
            if diff == 0 {
                // swap lp-space and sp-file (we don't leave any spare spaces)
                (disk[lp], disk[sp]) = (disk[sp].clone(), disk[lp].clone());
            }
            else {
                // put sp-file to lp-space and fill hole after sf-file with Space of same size
                (disk[lp], disk[sp]) = (disk[sp].clone(), DS::Space(i32::abs(free_space_size-diff)));
                // the file did not fill the original hole fully so we add new space of the size diff
                disk.insert(lp+1, DS::Space(diff))
            }
        }

        // println!("{}/{}", lp, disk.len());
        lp += 1;
    }
}

fn main() {
    let mut input = fs::read_to_string("input.txt").expect("Problem reading file");
    // let mut input = fs::read_to_string("small_input.txt").expect("Problem reading file");
    input.pop();

    let nums: Vec<usize> = input.split("").filter(|c| *c != "").map(|x| x.parse().unwrap()).collect();

    let sum: usize = nums.iter().sum();

    let mut disk: Vec<i32> = vec![-1; sum];


    println!("new len {}", disk.len());

    let mut dsdisk: Vec<DS> = copy_old(&nums, &mut disk);

    compress(&mut disk);
    let mut checksum: usize = 0;
    for i in 0..disk.len() {
        if disk[i] == -1 { continue; }

        checksum += i*(disk[i] as usize);
    }
    println!("Part1 {}", checksum);

    compress_ds(&mut dsdisk);

    let mut checksum_p2: usize = 0;

    let mut i: usize = 0;
    for f in 0..dsdisk.len() {
        match dsdisk[f] {
            DS::File(id, size) => {
                for _ in 0..size {
                    checksum_p2 += i*(id as usize);
                    i+=1;
                }
            },
            DS::Space(size) => { 
                i += size as usize; },
        }
    }

    println!("Part2 {}", checksum_p2);
}
