use std::fs::read_to_string;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Visit {
    dir:Direction,
    till_last_turn:u8,
    heat_loss:usize,
}

#[derive(Clone)]
struct Block {
    visits:HashSet<Visit>,
    heat_absorption:usize,
}

// start from a block in the grid
// for every visit add to the near blocks the possible ones
fn main() {
    let input = read_to_string("/home/etonit/advent-of-code2023/17day/input").unwrap();
    let test = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";

    let mut grid:Vec<Vec<Block>> = input
        .split("\n")
        .filter(|e| e.len() != 0 )
        .map(|e| e
             .chars()
             .map(|n| Block{ visits:HashSet::new(), heat_absorption: n.to_string().parse().unwrap() } )
             .collect())
        .collect();

    grid[0][0].visits = [
        Visit{ heat_loss:0, till_last_turn:1, dir:Direction::Right }, 
        Visit{ heat_loss:0, till_last_turn:1, dir:Direction::Down }].into();


    let mut smalles_so_far = usize::MAX;
    let mut changed = HashSet::new();
    changed.insert((0,0));
    while let Some((yy,xx)) = changed.iter().next().clone() {
        let y = *yy;
        let x = *xx;
        changed.remove(&(y,x));
        println!("{}", changed.len());

        for vis in &grid[y][x].visits.clone() {
            add_possible_mov(vis, &mut grid, y, x, &mut changed, &mut smalles_so_far);
        }
    }

    println!("{:?}",smalles_so_far);
}

fn add_possible_mov(
    b:&Visit, 
    grid:&mut Vec<Vec<Block>>, 
    y:usize, x:usize, 
    changed:&mut HashSet<(usize,usize)>,
    smallest: &mut usize) {
    match b.dir {
        Direction::Up => {
            if b.till_last_turn < 3 {
                insert(grid, &[Direction::Down],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Down,Direction::Up],y,x,b, changed, smallest);
            }
        }
        Direction::Down => {
            if b.till_last_turn < 3 {
                insert(grid, &[Direction::Up],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Up,Direction::Down],y,x,b, changed, smallest);
            }
        }
        Direction::Right => {
            if b.till_last_turn < 3 {
                insert(grid, &[Direction::Left],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Left,Direction::Right],y,x,b, changed, smallest);
            }
        }
        Direction::Left => {
            if b.till_last_turn < 3 {
                insert(grid, &[Direction::Right],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Right,Direction::Left],y,x,b, changed, smallest);
            }
        }
    }
}

fn insert(
    grid:&mut Vec<Vec<Block>>, 
    banned:&[Direction], 
    y:usize, x:usize, 
    b:&Visit, 
    changed:&mut HashSet<(usize,usize)>,
    smallest:&mut usize) {

    let up = if b.dir == Direction::Up { b.till_last_turn + 1 } else { 1 };
    let down = if b.dir == Direction::Down { b.till_last_turn + 1 } else { 1 };
    let left = if b.dir == Direction::Left { b.till_last_turn + 1 } else { 1 };
    let right = if b.dir == Direction::Right { b.till_last_turn + 1 } else { 1 };

    if y > 0 && !banned.contains(&Direction::Up) {
        change(b, grid, y-1, x, up, Direction::Up, changed, smallest);
    }
    if y < grid.len()-1 && !banned.contains(&Direction::Down) {
        change(b, grid, y+1, x, down, Direction::Down, changed, smallest);
    }

    if x > 0 && !banned.contains(&Direction::Left) {
        change(b, grid, y, x-1, left, Direction::Left, changed, smallest);
    }
    if x < grid[y].len()-1 && !banned.contains(&Direction::Right) {
        change(b, grid, y, x+1, right, Direction::Right, changed, smallest);
    }
}
// forgot to add the diection of movement for change
fn change(
    b:&Visit, 
    grid:&mut Vec<Vec<Block>>, 
    y:usize, x:usize,
    diff:u8, 
    dir:Direction,
    changed:&mut HashSet<(usize,usize)>,
    smallest:&mut usize) {

    let Visit{ heat_loss, .. } = b.clone();
    let to_add = Visit{
        dir,
        till_last_turn:diff,
        heat_loss:heat_loss + grid[y][x].heat_absorption,
    };

    if (y,x) == (grid.len()-1, grid[y].len()-1) {
        *smallest = min(*smallest, to_add.heat_loss);
        return
    }


    if  to_add.heat_loss > *smallest 
        || grid[y][x].visits.iter().any(|e| e.heat_loss <= to_add.heat_loss && e.dir == to_add.dir
                                    && e.till_last_turn <= to_add.till_last_turn) {
        return
    }

    //println!("{} {:?} {:?}", changed.len(), (y,x), to_add);

    if grid[y][x].visits.insert(to_add) {
        changed.insert((y,x));
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}



/*
2413432311323
2>>34^>>>1323
4 1 3 2 3 1
14

3215453535623
32v>>>35v5623
1 5 4 5 3
32

3255245654254
32552456v>>54
5 4 2
43

3446585845452
3446585845v52
4
47

4546657867536
4546657867v>6
5 3
55

1438598798454
14385987984v4
5
60

4457876987766
44578769877v6
6
66

3637877979653
36378779796v>
5 3
74

4654967986887
465496798688v
7
81

4564679986453
456467998645v
3
84

1224686865563
12246868655<v
6 3
93

2546548887735
25465488877v5
3 5
101

4322674655533
43226746555v>
3 3
107 - 3 - 2
 */
/*

*/
