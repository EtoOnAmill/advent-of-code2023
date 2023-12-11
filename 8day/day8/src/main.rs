use std::fs;
use std::collections::HashMap;
use num::integer::lcm;
use num::BigUint;

fn ends(s:&str, with:char) -> bool {
    *s.as_bytes().last().unwrap() as char == with
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/8day/input").unwrap();
    let test = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n";

    let ins_map:Vec<&str> = input
        .split("\n\n")
        .collect();
    let instruction = ins_map[0];
    let map = ins_map[1];
    println!("{:?}",instruction);

    let mut hash:HashMap<&str,(&str,&str)> = HashMap::new();
    for line in map.split("\n").filter(|e| e != &"") {
        let [key, left, right] = line.split(|e| match e {
            ' ' | '=' | '(' | ')' | ',' => true,
            _ => false
        }).filter(|e| e != &"").collect::<Vec<_>>()[..] else {panic!("failed to parse {}",line)};
        hash.insert(key,(left,right));
    }

    let mut curr_locs = hash
        .iter()
        .filter_map(|e| if ends(e.0,'A') {Some(*e.0)} else {None}).collect::<Vec<&str>>();

    let mut llcm = BigUint::new(vec![1]);
    for curr_loc in curr_locs.iter_mut() {

        let mut steps = BigUint::new(vec![0]);
        let mut instr_iter = instruction.chars().cycle();
        for curr_instr in instr_iter {

            match curr_instr {
                'L' => { 
                    *curr_loc = hash.get(curr_loc).unwrap().0
                }
                'R' => { 
                    *curr_loc = hash.get(curr_loc).unwrap().1
                }
                _ => panic!()

            }

            steps = steps + 1u32;
            if ends(curr_loc, 'Z') {break}
        }
        llcm = lcm(llcm, steps);
    }
    println!("{}",llcm);
}
