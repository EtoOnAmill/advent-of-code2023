use std::fs;
use std::cmp::{min, max};


fn empty_columns(lines:&Vec<Vec<char>>) -> Vec<usize> {
    let mut ret = Vec::new();
    for colum in 0..lines[0].len() {
        let mut empty = true;
        for row in 0..lines.len() {
            empty = empty && lines[row][colum] == '.';
        }
        if empty { ret.push(colum) }
    }

    ret
}
fn empty_rows(lines:&Vec<Vec<char>>) -> Vec<usize> {
    let mut ret = Vec::new();

    for row in 0..lines.len() {
        if lines[row].iter().all(|e| *e == '.') { ret.push(row) }
    }

    ret
}
fn print_grid(g:&Vec<Vec<char>>) {
    for line in g {
        for c in line {
            print!("{}", c);
        }
        print!("\n");
    }
}
fn distance(p1:&(usize,usize), p2:&(usize,usize), expansions_column:&Vec<usize>, expansions_row:&Vec<usize>) -> usize {
    let y1_exp = expansions_row.iter().filter(|e| **e < p1.0).count();
    let y2_exp = expansions_row.iter().filter(|e| **e < p2.0).count();

    let x1_exp = expansions_column.iter().filter(|e| **e < p1.1).count();
    let x2_exp = expansions_column.iter().filter(|e| **e < p2.1).count();

    let exp = 1000000;

    let y1 = p1.0 + y1_exp * exp - y1_exp;
    let y2 = p2.0 + y2_exp * exp - y2_exp;

    let x1 = p1.1 + x1_exp * exp - x1_exp;
    let x2 = p2.1 + x2_exp * exp - x2_exp;


    let xd = max(x1, x2) - min(x1, x2);
    let yd = max(y1, y2) - min(y1, y2);
    let distance = xd + yd;
    /*
    println!("p1:{:?} p2:{:?} | dist: {}  dx:{: >3} dy:{: >3} | x1:{: >3} x1-exp:{: >3} | x2:{: >3} x2-exp:{: >3} | y1:{: >3} y1-exp:{: >3} | y2:{: >3} y2-exp:{: >3}"
             , p1,p2
             , distance
             , xd,yd
             , x1,x1_exp
             , x2,x2_exp
             , y1,y1_exp
             , y2,y2_exp);
    */

    distance
}
fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/11day/input").unwrap();
    let test = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
    let mut grid = input
        .split("\n")
        .filter_map(|line| if line.len() != 0
                    { Some(line.chars().collect::<Vec<char>>()) }
                    else { None })
        .collect::<Vec<Vec<char>>>();

    let e_col = empty_columns(&grid);
    let e_row = empty_rows(&grid);
    println!("x: {:?}\ny: {:?}", e_col, e_row);
    print_grid(&grid);

    let mut galaxies = Vec::new();
    for (iy, line) in grid.iter().enumerate() {
        for (ix, chr) in line.iter().enumerate() {
            if *chr == '#' { galaxies.push((iy, ix)) };
        }
    }
    println!("{:?}", galaxies);

    let mut all_pairs = Vec::new();
    for i in 0..(galaxies.len()-1) {
        for j in (i+1)..galaxies.len() {
            all_pairs.push((galaxies[i], galaxies[j]));
        }
    }
    println!("{:?}", all_pairs);

    let res = all_pairs.iter()
        .fold(0, |acc, (g1,g2)| distance(g1,g2,&e_col,&e_row) + acc);
    println!("{}", res);
}
