use std::fs;
use std::ops::Range;
use std::cmp::min;

struct Stepping{
    data:Vec<usize>,
    idx:usize,
    curr_range:Range<usize>
}
impl Iterator for Stepping {
    type Item=usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(n) = self.curr_range.next() {
            Some(n)
        } else {
            self.idx += 2;
            let next_start = self.data.get(self.idx)?;
            let next_size = self.data.get(self.idx+1)?;
            self.curr_range = *next_start..(next_start+next_size);
            self.curr_range.next()
        }
    }
}
impl Stepping {
    fn new (v:Vec<usize>)->Stepping {
        Stepping {
            idx:0,
            curr_range:v[0]..v[1],
            data:v
        }
    }
}

fn main() {
    let bind = fs::read_to_string("/home/etonit/advent-of-code2023/5day/input")
        .unwrap();

    let sectors:Vec<&str> = bind.split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .collect::<Vec<&str>>();

    let seeds_builder:Vec<usize> = sectors[0]
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let mut maps = Vec::new();
    for map in sectors[1..].into_iter() {
        let mut curr_map = Vec::new();

        let mut dest_src_rang = map.split("\n");
        dest_src_rang.next();

        let cln = dest_src_rang.clone();
        for e in cln.map(|dsr| dsr.split(' ').collect::<Vec<&str>>()) {
            let [dest,src,rang] = e[..] else {break};
            let [d,s,r]:[usize;3] = [dest.parse().unwrap(),src.parse().unwrap(),rang.parse().unwrap()];

            curr_map.push((d,s,r));
        }
        maps.push(curr_map);
    }

    let mut smallest = usize::MAX;
    for seed in Stepping::new(seeds_builder){
        let mut src = seed;
        for map in &maps[..] {
            for (d,s,r) in &map[..]{
                if src < s+r && src >= *s {
                    src = d + src - s;
                }
            }
        }
        smallest = min(smallest, src);
    }
    println!("{}",smallest);
}
