use std::fs::read_to_string;
use std::cmp::max;
use std::collections::{ VecDeque, HashMap, HashSet};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Beam {
    xy:(usize,usize),
    dir:Direction,
}

fn move_ray(Beam{ xy, dir }:&Beam, grid:&Vec<Vec<char>>) -> Vec<Beam> {
    let row_n = grid.len();
    let column_n = grid[0].len();
    let (x, y) = xy.clone();

    let mut ret = Vec::new();
    match dir {
        Direction::Up if xy.1 != 0 => {
            match grid[y-1][x] {
                '\\' => { ret.push(Beam{ xy:(x,y-1), dir:Direction::Left }) }
                '/' => { ret.push(Beam{ xy:(x,y-1), dir:Direction::Right }) }
                '-' => { 
                    ret.push(Beam{ xy:(x,y-1), dir:Direction::Left });
                    ret.push(Beam{ xy:(x,y-1), dir:Direction::Right });
                }
                _ => { ret.push(Beam{ xy:(x,y-1), dir:Direction::Up }) }
            }
        }
        Direction::Down if xy.1 != row_n - 1 => {
            match grid[y+1][x] {
                '\\' => { ret.push(Beam{ xy:(x,y+1), dir:Direction::Right }) }
                '/' => { ret.push(Beam{ xy:(x,y+1), dir:Direction::Left }) }
                '-' => { 
                    ret.push(Beam{ xy:(x,y+1), dir:Direction::Left });
                    ret.push(Beam{ xy:(x,y+1), dir:Direction::Right });
                }
                _ => { ret.push(Beam{ xy:(x,y+1), dir:Direction::Down }) }
            }
        }
        Direction::Left if xy.0 != 0 => {
            match grid[y][x-1] {
                '\\' => { ret.push(Beam{ xy:(x-1,y), dir:Direction::Up }) }
                '/' => { ret.push(Beam{ xy:(x-1,y), dir:Direction::Down }) }
                '|' => { 
                    ret.push(Beam{ xy:(x-1,y), dir:Direction::Up });
                    ret.push(Beam{ xy:(x-1,y), dir:Direction::Down });
                }
                _ => { ret.push(Beam{ xy:(x-1,y), dir:Direction::Left }) }
            }
        }
        Direction::Right if xy.0 != column_n - 1 => {
            match grid[y][x+1] {
                '\\' => { ret.push(Beam{ xy:(x+1,y), dir:Direction::Down }) }
                '/' => { ret.push(Beam{ xy:(x+1,y), dir:Direction::Up }) }
                '|' => { 
                    ret.push(Beam{ xy:(x+1,y), dir:Direction::Up });
                    ret.push(Beam{ xy:(x+1,y), dir:Direction::Down });
                }
                _ => { ret.push(Beam{ xy:(x+1,y), dir:Direction::Right }) }
            }
        }
        _ => {}
    }

    ret
}

fn energyze_cells(grid:&Vec<Vec<char>>, c:(usize,usize), d:Direction) -> usize {
    let mut beams = VecDeque::new();
    beams.push_back(Beam{ xy:c, dir:d.clone() });

    let mut map:HashMap<Beam, usize> = HashMap::new();
    while beams.len() != 0 {
        // add curr position to map
        // look in the direction of the beam
        // if outside grid continue
        // if splitter split accordingly
        // if mirror change direction accordingly
        let b = beams.pop_front().unwrap();
        let Beam{ xy, dir } = b.clone();
        if let Some(passed) = map.get_mut(&b) {
            *passed += 1;
            continue;
        } else {
            map.insert(b.clone(), 1);
        }

        for ray in move_ray(&b, &grid) {
            beams.push_back(ray);
        }
    }

    let mut unique = HashSet::new();
    map.iter().for_each(|e| { unique.insert(e.0.xy.clone()); });
    //println!("corner {:?} dir {:?} tot {}",c,d, unique.len());

    unique.len()
}

fn main() {
    let input = read_to_string("/home/etonit/advent-of-code2023/16day/input").unwrap();
    let test = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

    let grid:Vec<Vec<char>> = input
        .split("\n")
        .filter(|e| e.len() != 0)
        .map(|e| e.chars().collect())
        .collect();
    let row_n = grid.len();
    let column_n = grid[0].len();

    // all left all righe all top all bottom
    let mut tot = 0;
    for row in 0..row_n {
        tot = max(tot, energyze_cells(&grid, (0,row), Direction::Right));
        tot = max(tot, energyze_cells(&grid, (column_n-1,row), Direction::Left));
    }

    for column in 0..column_n {
        tot = max(tot, energyze_cells(&grid, (column,0), Direction::Down));
        tot = max(tot, energyze_cells(&grid, (column,row_n-1), Direction::Up));
    }


    println!("{}",tot);
}
