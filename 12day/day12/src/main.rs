use std::fs;
// . functioning spring
// # broken spring
// ? unknown spring
//      springs broken_arrangement
//      ..##.#....## 2,1,2

fn binary_idxs(v:usize) -> Vec<bool> {
    let mut ret = Vec::new();

    for c in format!("{:0>20b}", v).chars().rev() {
        if c == '1' { ret.push(true) }
        else { ret.push(false) }
    }

    ret
}

fn check_validity(record:&str, parity:&[usize]) -> bool {
    let mut broken = Vec::new();
    let mut continous = false;
    for c in record.chars() {
        if c == '#' && continous {
            *broken.last_mut().unwrap() += 1;
        } else if c == '#' {
            continous = true;
            broken.push(1);
        } else {
            continous = false;
        }
    }
    broken.as_slice() == parity
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/12day/input").unwrap();
    let test = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    let lines = input
        .split("\n")
        .filter(|e| e.len() != 0 )
        .map(|e| e.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut tot = 0;
    for line in &lines {
        let curr = tot;
        let record = line[0];
        let parity = line[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        print!("{:?} | {:?}", record, parity);

        let broken_known = record.chars().filter(|e| *e == '#').count();
        let broken_declared = parity.iter().fold(0, |acc,e| acc + *e);
        if broken_known == broken_declared { 
            tot += 1;
            println!(" {}", tot - curr);
            continue;
        }
        for n in 0..2_usize.pow(record.chars().filter(|e| *e == '?').count() as u32) {
            let binary = binary_idxs(n);

            let broken_to_add = binary.iter().filter(|e| **e).count();
            if broken_to_add != broken_declared - broken_known { continue }
            //println!("{}", binary.iter().rev().fold(0, |acc,e| if *e { acc * 2 + *e as u32 } else { acc * 2 }));

            let mut chars = record.chars().collect::<Vec<char>>();
            let mut b_idx = 0; 
            for i in 0..chars.len() {
                if chars[i] == '?' {
                    chars[i] = if binary[b_idx] { '#' } else { '.' };
                    b_idx += 1;
                }
            }
            if check_validity(chars.iter().collect::<String>().as_str(), parity.as_slice()) { tot += 1; }
        }
        println!(" {}", tot - curr);
    }

    println!("{}", tot);
}
