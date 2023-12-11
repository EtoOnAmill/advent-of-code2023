use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

const CARDS:[char;13] = ['J','2','3','4','5','6','7','8','9','T','Q','K','A']; 

fn card_points(c:&char) -> usize{
    for (i, cc) in (1..).zip(CARDS.iter()){
        if cc == c { return i }
    }
    return 0;
}

#[derive(PartialEq)]
struct HandType {
    oootf:[char; 10],
    hand_power:u8
}
impl HandType {
    fn to_str(&self) -> String {
        self.oootf.iter().collect()
    }
    fn new(s:&str) -> HandType{
        let mut cards:HashMap<char,usize> = HashMap::new();
        for c in s.chars() {
            if let Some(n) = cards.get(&c) {
                cards.insert(c, n+1);
            } else { 
                cards.insert(c, 1);
            }
        }
        // fancy smancy sort
        let mut sorted = cards.iter().collect::<Vec<(&char,&usize)>>();
        //println!("{:?}",sorted);

        // 6 - five of a kind
        // 5 - four of a kind
        // 4 - full house (triplet + pair)
        // 3 - three of a kind
        // 2 - two pairs
        // 1 - one pair

        let mut hand_power:u8 = 0;
        for (c,n) in sorted {
            match n {
                _ if *c == 'J' => {}
                5 => {hand_power = 6},
                4 => {hand_power = 5},
                3 => {hand_power += 3},
                2 => {hand_power += 1}
                1 | 0 => {}
                _ => panic!("{}",n)
            }
        }
        if let Some(n) = cards.get(&'J') {
            match (hand_power,n) {
                (0,5) => hand_power = 6,
                (0,4) => hand_power = 6,
                (1,3) => hand_power = 6, // 1 pair + 3J
                (3,2) => hand_power = 6, // 3 same + 2J
                (5,1) => hand_power = 6, // 4 same + 1J
                (0,3) => hand_power = 5,
                (1,2) => hand_power = 5, // 1 pair + 2J
                (3,1) => hand_power = 5, // 3 same + 1J
                (2,1) => hand_power = 4, // 2 pairs + 1J
                (0,2) => hand_power = 3,
                (1,1) => hand_power = 3, // 1 pair + 1J
                (0,1) => hand_power = 1,
                _ => panic!("{} {}",hand_power,n)
            }
        }

        let mut oootf = [' ';10];
        for (i,cc) in &s.chars().enumerate().collect::<Vec<(usize,char)>>()[0..5] {
            oootf[*i] = *cc;
        }
        HandType{oootf,hand_power}
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other:&HandType) -> Option<Ordering> {
        if self.hand_power == other.hand_power {
            let test = self.oootf
                .iter()
                .zip(other.oootf.iter())
                .find_map(
                    |(st,nd)| {
                        let res = card_points(st).cmp(&card_points(nd));
                        if let Ordering::Equal = res {None}
                        else {Some(res)}
                    });
            if let None = test {Some(Ordering::Equal)}
            else {test}
        } else {
            self.hand_power.partial_cmp(&other.hand_power)
        }
    }
}

fn main() {
    let test = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    let file = fs::read_to_string("/home/etonit/advent-of-code2023/7day/input").unwrap();
    let mut lines:Vec<(HandType,usize)> = file
        .split("\n")
        .filter(|e| e != &"")
        .map(|e| {
            let [h,bid] = e
                .split(" ")
                .filter(|e| e != &"")
                .collect::<Vec<&str>>()[0..2] else {panic!()};
            let han = HandType::new(h); 
            let bid = bid.parse::<usize>().unwrap();
            (han,bid)
        }).collect();
    lines.sort_by(|st,nd| st.0.partial_cmp(&nd.0).unwrap());


    let mut acc = 0;
    for (i,(h,b)) in lines.iter().enumerate() {
        acc += (1+i) * b;
        println!("{}: {}-{} _ {}",i,h.to_str(),b,acc);
    }
    println!("{}",acc);
}
