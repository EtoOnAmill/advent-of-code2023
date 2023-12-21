use std::fs::read_to_string;
use std::cmp::{ max, min };
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    No
}
impl Direction {
    fn from(s:&str) -> Self {
       match s {
           "R" | "0" => Direction::Right,
           "D" | "1" => Direction::Down,
           "L" | "2" => Direction::Left,
           "U" | "3" => Direction::Up,
           _ => panic!("{:?}",s)
       }
    }
}

struct DigInst {
    dir:Direction,
    distance:isize,
}
impl DigInst {
    fn from(s:&str) -> Self {
        let [d,l,c] = s.split(" ").collect::<Vec<_>>()[..] else { panic!("passed non valid string {:?}",s)};
        let dir = Direction::from(d);
        let distance = l.parse().unwrap();
        
        
        let mut vv = c.chars().collect::<Vec<char>>() else { panic!("passed non valid string {:?}",s)};
        //let dir = Direction::from(&vv[7].to_string());
        //let distance = isize::from_str_radix(&vv[2..7].iter().collect::<String>(), 16).unwrap();
        println!("{:?} {}", dir, distance);

        Self {
            dir,
            distance,
        }
    }
}

fn main() {
    let input = read_to_string("/home/etonit/advent-of-code2023/18day/input").unwrap();
    let test = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

    let instructions:Vec<DigInst> = input
        .split("\n")
        .filter(|e| e.len() != 0)
        .map(|e| DigInst::from(e))
        .collect();

    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    
    let mut curr_x = 0;
    let mut curr_y = 0;
    for DigInst{ distance, dir } in &instructions {
        match dir {
            Direction::Left => {
                curr_x -= distance;
            }
            Direction::Right => {
                curr_x += distance;
            }
            Direction::Down => {
                curr_y += distance;
            }
            Direction::Up => {
                curr_y -= distance;
            }
            Direction::No => {}
        }
        max_x = max(max_x, curr_x);
        max_y = max(max_y, curr_y);

        min_x = min(min_x, curr_x);
        min_y = min(min_y, curr_y);
    }
    
    let mut up:Vec<(RangeInclusive<usize>, usize)> = Vec::new();
    let mut down:Vec<(RangeInclusive<usize>, usize)> = Vec::new();
    let mut left:Vec<(RangeInclusive<usize>, usize)> = Vec::new();
    let mut right:Vec<(RangeInclusive<usize>, usize)> = Vec::new();

    let mut y = (min_y * (-1)) as usize;
    let mut x = (min_x * (-1)) as usize;
    for DigInst{ distance, dir } in instructions {
        let distance = distance as usize;
        //println!("x {} y {} dist {} dir {:?}", x,y,distance, dir);
        match dir {
            Direction::Down => {
                down.push((y..=(y+distance), x));
                y += distance;
            }
            Direction::Up => {
                up.push(((y-distance)..=y, x));
                y -= distance;
            }
            Direction::Left => {
                left.push(((x-distance)..=x, y));
                x -= distance;
            }
            Direction::Right => {
                right.push((x..=(x+distance), y));
                x += distance;
            }
            Direction::No => {}
        }
    }

    //print_mov(&mov_grid);
    println!("x: {} {}\ny: {} {}\n",min_x,max_x, min_y,max_y);

    let yy = (max_y + min_y.abs()) as usize;
    let xx = (max_x + min_x.abs()) as usize;

    let mut area = 0;
    for (y_rang, x_coord) in &up {

        for y in y_rang.clone() {
            let mut y_tot = 1;

            for x in (*x_coord+1)..=xx {
                y_tot += 1;

                if down.iter().any(|e| e.0.contains(&y) && e.1 == x) {
                    println!("found up at y {} x {}", y, x);
                    break
                }

                if up.iter().any(|e| e.0.contains(&y) && e.1 == x) {
                    println!("found down at y {} x {}", y, x);
                    break
                }
            }
            area += y_tot;
            println!("y {} x {} tot {}", y, x_coord, y_tot);
        }
    }

    //print_grid(&grid);
    println!("{}", area);
}
fn print_mov(v:&Vec<Vec<Direction>>) {
    println!("{}", v
             .iter()
             .map(|line| line.iter().map(|d| match d {
                 Direction::Up => {
                     "^"
                 }
                 Direction::Down => {
                     "v"
                 }
                 Direction::Left => {
                     "<"
                 }
                 Direction::Right => {
                     ">"
                 }
                 Direction::No => {
                     "."
                 }
             })
                  .collect::<String>())
             .collect::<Vec<_>>()
             .join("\n")
    );
}
fn print_grid(v:&Vec<Vec<bool>>) {
    println!("{}", 
             v
             .iter()
             .map(|ee| ee
                  .iter()
                  .map(|e| if *e { "#".to_string() } else { ".".to_string() })
                  .collect::<String>())
             .collect::<Vec<_>>()
             .join("\n"));
}
