use std::fs;
use std::collections::HashMap;

enum Direction {
    South,
    North,
    East,
    West
}

fn bubble(grid:&mut Vec<Vec<char>>, y:usize, x:usize, dir:Direction) {
    if grid[y][x] != 'O' { return }
    match dir {
        Direction::South => {
            for next_y in (y+1)..grid.len() {
                if grid[next_y][x] == '.' {
                    grid[next_y - 1][x] = '.';
                    grid[next_y][x] = 'O';
                } else {
                    break;
                }
            }
        }
        Direction::North => {
            for next_y in (0..y).rev() {
                if grid[next_y][x] == '.' {
                    grid[next_y + 1][x] = '.';
                    grid[next_y][x] = 'O';
                } else {
                    break;
                }
            }
        }
        Direction::East => {
            for next_x in (x+1)..grid[x].len() {
                if grid[y][next_x] == '.' {
                    grid[y][next_x - 1] = '.';
                    grid[y][next_x] = 'O';
                } else {
                    break;
                }
            }
        }
        Direction::West => {
            for next_x in (0..x).rev() {
                if grid[y][next_x] == '.' {
                    grid[y][next_x + 1] = '.';
                    grid[y][next_x] = 'O';
                } else {
                    break;
                }
            }
        }
    }
}

fn cycle(grid:&mut Vec<Vec<char>>) {
    // north
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            bubble(grid, y, x, Direction::North);
        }
    }
    // west
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            bubble(grid, y, x, Direction::West);
        }
    }
    // south
    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            bubble(grid, y, x, Direction::South);
        }
    }
    // east
    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            bubble(grid, y, x, Direction::East);
        }
    }
}
fn print_grid(g:&Vec<Vec<char>>) {
    println!("{}\n",
             g
             .iter()
             .fold(String::new(),
             |acc, e| acc + &e.iter().collect::<String>() + "\n"));
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/14day/input").unwrap();
    let test = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

    let original_grid:Vec<Vec<char>> = input
        .split("\n")
        .filter(|e| e.len() != 0)
        .map(|e| e.chars().collect())
        .collect();

    let mut grid = original_grid.clone();
    // save every found position + their idx for when they are found
    let mut map = HashMap::new();
    // (start, end)
    let mut cycle_len = (0, 0);
    //cycle until you find an alraedy found position
    for n in 0.. {
        // save the starting idx and the cycle length
        if let Some(nn) = map.insert(grid.clone(), n) {
            cycle_len = (nn, n - nn);
            break;
        }
        cycle(&mut grid);
    }

    let total_cycles = 1000000000;
    // find how many cycles went on before the circular position is found
    let repeat_cycles = total_cycles - cycle_len.0;
    // find the last position by taking the modulus and adding the start of the circular cycle
    // 0 1 2 3 4 5 2 3 4 5 2 - (10-2)%4+2 = 8%4+2 = 2
    // 0 1 2 3 4 5 6 7 3 4 5 - (10-3)%5+3 = 7%5+3 = 5
    let cycle_idx = cycle_len.0 + repeat_cycles % cycle_len.1;
    // find the position that has idx = last position
    let last_grid = map.iter().find_map(|(k,v)| if *v==cycle_idx {Some(k.clone())} else {None}).unwrap();

    let load = last_grid.iter()
        .rev()
        .enumerate()
        .map(|(i,e)| (i+1,e.iter().filter(|e| **e=='O').count()))
        .fold(0, |acc,(i,e)| acc + i*e);
    
    println!("{}", load);
}
