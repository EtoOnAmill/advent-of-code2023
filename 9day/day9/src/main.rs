use std::fs;

fn derive(ns:&[isize]) -> Vec<isize> {
    let mut ret = Vec::new();
    for w in ns.windows(2) {
        let [st,nd] = w else {panic!()};
        ret.push(nd - st);
    }
    ret
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/9day/input").unwrap();
    let test = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    let lines = input
        .split("\n")
        .filter(|e| e.len() != 0)
        .collect::<Vec<_>>();

    let mut tot = 0;
    for line in lines {
        let ns = line
            .split(" ")
            .filter_map(|e| if e.len() != 0 {e.parse::<isize>().ok()} else {None})
            .collect::<Vec<isize>>()
            .into_iter()
            .rev()
            .collect::<Vec<isize>>();
        
        let mut ret = vec![ns];

        loop {
            let last = ret.last().unwrap();
            if last.iter().all(|e| *e == 0) {break}
            ret.push(derive(&last[..]));
        }

        tot += ret
            .iter()
            .rev()
            .fold(0, |acc,e| e.last().unwrap() + acc);
    }

    println!("{tot}");
}
