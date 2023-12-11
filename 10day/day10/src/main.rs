use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Directions {
    Top,
    Left,
    Bottom,
    Right,
//   TopLeft, // J
//   TopRight, // L
//
//   BottomLeft, // 7
//   BottomRight, // F
//
//   TopBottom, // |
//   RightLeft, // -
//   
//   None,
}

fn get_s(grid:&Vec<Vec<char>>) -> (usize,usize) {
    let mut ret = (0,0);

    // find S 
    for iy in 0..grid.len() {
        for ix in 0..grid[iy].len() {
            let c = grid[iy][ix];
            if c == 'S' {return (iy,ix)}
        }
    }

    ret
}

fn get_dirs(c:char) -> [Directions; 2]  {
    match c {
        'J' => [Directions::Top, Directions::Left],
        'L' => [Directions::Top, Directions::Right],
        '7' => [Directions::Bottom, Directions::Left],
        'F' => [Directions::Bottom, Directions::Right],
        '|' => [Directions::Top, Directions::Bottom],
        '-' => [Directions::Left, Directions::Right],
        _ => panic!()
    }
}

fn opposite(d:Directions) -> Directions {
    match d {
        Directions::Top => Directions::Bottom,
        Directions::Bottom => Directions::Top,
        Directions::Left => Directions::Right,
        Directions::Right => Directions::Left,
    }
}

fn follow(grid:&mut Vec<Vec<char>>, pos:(usize,usize),  last_mov:Directions, steps:usize) -> usize {
    let curr = grid[pos.0][pos.1];
    if curr == 'S' {return steps + 1}

    match curr {
        'J' => grid[pos.0][pos.1] = '┙',
        '7' => grid[pos.0][pos.1] = '┑',
        'L' => grid[pos.0][pos.1] = '└',
        'F' => grid[pos.0][pos.1] = '┌',
        '|' => grid[pos.0][pos.1] = '│',
        '-' => grid[pos.0][pos.1] = '─',
        _ => panic!()
    }

    let new_mov = *get_dirs(curr).iter().filter(|e| **e != opposite(last_mov)).next().unwrap();

    match new_mov {
        Directions::Top => follow(grid, (pos.0 - 1,pos.1), new_mov, steps+1),
        Directions::Bottom => follow(grid, (pos.0 + 1,pos.1), new_mov, steps+1),
        Directions::Left => follow(grid, (pos.0,pos.1 - 1), new_mov, steps+1),
        Directions::Right => follow(grid, (pos.0,pos.1 + 1), new_mov, steps+1),
    }
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/10day/input").unwrap();
    let mut grid:Vec<Vec<char>> = input
        .split("\n")
        .map(|e| e.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = get_s(&grid);
    let tot_loop = follow(&mut grid, (start.0, start.1 -1), Directions::Left, 0);
    println!("{} {}", tot_loop, tot_loop / 2);

    println!("{}", grid
             .iter()
             .map(|e| e.iter().collect::<String>())
             .collect::<Vec<String>>()
             .join("\n"));

    let mut top = false;
    let mut bottom = false;
    let mut area = 0;
    for line in grid.iter() {
        for chr in line.iter() {

            match chr {
                '┙' | 'S' => top = !top,
                '└' => top = !top,
                '┑' => bottom = !bottom,
                '┌' => bottom = !bottom,
                '│' => {bottom = !bottom; top = !top},
                _ if top && bottom => {area += 1}
                _ => {}
            }
       }
    }
    println!("{}", area);
}
