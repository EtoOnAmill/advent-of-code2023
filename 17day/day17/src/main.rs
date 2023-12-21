use std::fs::read_to_string;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::VecDeque;

const MIN:u8 = 4;
const MAX:u8 = 10;

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
    visits:Vec<Visit>,
    heat_absorption:usize,
}

fn find_lowest_heat_loss(
    grid:&mut Vec<Vec<Block>>, 
    dir:Direction, 
    y:usize, x:usize,
    heat_loss:usize,
    lowest:&mut Option<usize>) -> Option<usize> {
    
    if x == grid.get(y)?.len() - 1 && y == grid.len() - 1 {
        let final_lowest = heat_loss+grid[y][x].heat_absorption;
        if lowest.iter().fold(true, |acc, e| *e > final_lowest) {
            return Some(final_lowest);
        }
    }

    let mut new_heat_loss = heat_loss;

    match dir {
        Direction::Up => {

            // change till you can turn
            forced_first(grid,
                         |yy| y.checked_sub(yy),
                         |_| Some(x),
                         &mut new_heat_loss);

            // propagate till max len
            for prop_y in MIN..MAX {
                let Some(new_y) = y.checked_sub(prop_y as usize) else { break };
                let till = prop_y + 1;

                new_heat_loss += grid.get(new_y)?.get(x)?.heat_absorption;

                if let None = 
                    change(grid, &mut new_heat_loss, new_y, x, Direction::Up, till) {
                    break
                }

                if let Some(new_lowest) = 
                    find_lowest_heat_loss(
                        grid, 
                        Direction::Right, 
                        new_y, x+1,
                        new_heat_loss,
                        lowest) {
                    *lowest = Some(new_lowest);
                }
                if x > 0 {
                    if let Some(new_lowest) = 
                        find_lowest_heat_loss(
                            grid, 
                            Direction::Left, 
                            new_y, x-1,
                            new_heat_loss,
                            lowest) {
                            *lowest = Some(new_lowest);
                        }
                }
            }

        }
        Direction::Down => {

            // change till you can turn
            forced_first(grid,
                         |yy| Some(y+yy),
                         |_| Some(x),
                         &mut new_heat_loss);


            // propagate till max len
            for prop_y in MIN..MAX {
                let new_y = y + prop_y as usize;
                let till = prop_y + 1;

                new_heat_loss += grid.get(new_y)?.get(x)?.heat_absorption;

                if let None = 
                    change(grid, &mut new_heat_loss, new_y, x, Direction::Down, till) {
                    break
                }

                if let Some(new_lowest) = 
                    find_lowest_heat_loss(
                        grid, 
                        Direction::Right, 
                        new_y, x+1,
                        new_heat_loss,
                        lowest) {
                    *lowest = Some(new_lowest);
                }
                if x > 0 {
                    if let Some(new_lowest) = 
                        find_lowest_heat_loss(
                            grid, 
                            Direction::Left, 
                            new_y, x-1,
                            new_heat_loss,
                            lowest) {
                            *lowest = Some(new_lowest);
                        }
                }
            }
        }
        Direction::Left => {

            // change till you can turn
            forced_first(grid,
                         |_| Some(y),
                         |xx| x.checked_sub(xx),
                         &mut new_heat_loss);


            // propagate till max len
            for prop_x in MIN..MAX {
                let Some(new_x) = x.checked_sub(prop_x as usize) else { break };
                let till = prop_x + 1;

                new_heat_loss += grid.get(y)?.get(new_x)?.heat_absorption;

                if let None = 
                    change(grid, &mut new_heat_loss, y, new_x, Direction::Left, till) {
                    break
                }

                if let Some(new_lowest) = 
                    find_lowest_heat_loss(
                        grid, 
                        Direction::Down, 
                        y+1, new_x,
                        new_heat_loss,
                        lowest) {
                    *lowest = Some(new_lowest);
                }
                if y > 0 {
                    if let Some(new_lowest) = 
                        find_lowest_heat_loss(
                            grid, 
                            Direction::Up, 
                            y-1, new_x,
                            new_heat_loss,
                            lowest) {
                            *lowest = Some(new_lowest);
                        }
                }
            }
        }
        Direction::Right => {

            // change till you can turn
            forced_first(grid,
                         |_| Some(y),
                         |xx| Some(x+xx),
                         &mut new_heat_loss);


            // propagate till max len
            forking_second(grid,
                           |_| Some(y),
                           |xx| Some(x+xx),
                           &mut new_heat_loss,
                           lowest,
                           Direction::Right);
        }
    }


    *lowest
}
fn forking_second(grid:&mut Vec<Vec<Block>>,
                  fun_y:impl Fn(usize) -> Option<usize>,
                  fun_x:impl Fn(usize) -> Option<usize>,
                  new_heat_loss:&mut usize,
                  lowest:&mut Option<usize>,
                  main_dir:Direction)
                  -> Option<()> {

    for prop in MIN..MAX {
        let Some(new_y) = fun_y(prop as usize) else { break };
        let Some(new_x) = fun_x(prop as usize) else { break };
        let till = prop + 1;

        *new_heat_loss += grid.get(new_y)?.get(new_x)?.heat_absorption;

        if let None = 
            change(grid, new_heat_loss, new_y, new_x, main_dir.clone(), till) {
                break
            }

        if new_x < grid[0].len() - 1 && splitting(&main_dir).contains(&Direction::Right) {
            if let Some(new_lowest) = 
                find_lowest_heat_loss(
                    grid, 
                    Direction::Right, 
                    new_y, new_x+1,
                    *new_heat_loss,
                    lowest) {
                    *lowest = Some(new_lowest);
                }
        }
        if new_y < grid.len() - 1 && splitting(&main_dir).contains(&Direction::Down) {
            if let Some(new_lowest) = 
                find_lowest_heat_loss(
                    grid, 
                    Direction::Right, 
                    new_y+1, new_x,
                    *new_heat_loss,
                    lowest) {
                    *lowest = Some(new_lowest);
                }
        }

        if new_y > 0 && splitting(&main_dir).contains(&Direction::Up) {
            if let Some(new_lowest) = 
                find_lowest_heat_loss(
                    grid, 
                    Direction::Right, 
                    new_y-1, new_x,
                    *new_heat_loss,
                    lowest) {
                    *lowest = Some(new_lowest);
                }
        }
        if new_x > 0 && splitting(&main_dir).contains(&Direction::Left) {
            if let Some(new_lowest) = 
                find_lowest_heat_loss(
                    grid, 
                    Direction::Left, 
                    new_y, new_x-1,
                    *new_heat_loss,
                    lowest) {
                    *lowest = Some(new_lowest);
                }
        }
    }

    Some(())
}
fn forced_first(
    grid:&mut Vec<Vec<Block>>, 
    fun_y: impl Fn(usize)->Option<usize>,
    fun_x: impl Fn(usize)->Option<usize>,
    new_heat_loss:&mut usize) -> Option<()> {
    for add in 0..MIN {
        let Some(new_y) = fun_y(add as usize) else { break };
        let Some(new_x) = fun_x(add as usize) else { break };

        let till = add + 1;

        *new_heat_loss += grid.get(new_y)?.get(new_x)?.heat_absorption;
        change(grid, new_heat_loss, new_y, new_x, Direction::Up, till)?
    }

    Some(())
}
fn change(
    grid:&mut Vec<Vec<Block>>, 
    new_heat_loss: &mut usize, 
    y:usize, x:usize, 
    dir: Direction,
    till:u8) 
    -> Option<()> {

    let new_visit = Visit{ 
        dir,
        till_last_turn: till,
        heat_loss: *new_heat_loss
    };

    if grid.get(y)?.get(x)?.visits.iter().any(
        |e| e.dir == new_visit.dir 
        && e.heat_loss <= new_visit.heat_loss
        && e.till_last_turn <= new_visit.till_last_turn
        && e.till_last_turn >= MIN)
    { return None }


    if !grid.get(y)?.get(x)?.visits.contains(&new_visit) {
        grid.get_mut(y)?.get_mut(x)?.visits.push(new_visit);
    }

    Some(())
}
fn splitting(d:&Direction) -> [Direction; 2] {
    match d {
        Direction::Right => [Direction::Down,Direction::Up],
        Direction::Left => [Direction::Down,Direction::Up],
        Direction::Up => [Direction::Left,Direction::Right],
        Direction::Down => [Direction::Left,Direction::Right],
    }
}
// start from a block in the grid
// for every visit add to the near blocks the possible ones
fn main() {
    let input = read_to_string("/home/etonit/advent-of-code2023/17day/input").unwrap();
    let test = 
        //"111111111111\n999999999991\n999999999991\n999999999991\n999999999991";
        "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";

    let mut grid:Vec<Vec<Block>> = test
        .split("\n")
        .filter(|e| e.len() != 0 )
        .map(|e| e
             .chars()
             .map(|n| Block{ visits:Vec::new(), heat_absorption: n.to_string().parse().unwrap() } )
             .collect())
        .collect();

    println!("{:?}",find_lowest_heat_loss(&mut grid, Direction::Right, 0, 0, 0, &mut None));
    /*
    grid:&mut Vec<Vec<Block>>, 
    dir:Direction, 
    y:usize, x:usize,
    heat_loss:usize,
    lowest:&mut Option<usize>) -> Option<usize> {

    grid[0][0].visits = vec![
        Visit{ heat_loss:0, till_last_turn:1, dir:Direction::Right }, 
        Visit{ heat_loss:0, till_last_turn:1, dir:Direction::Down }];


    let mut smalles_so_far = usize::MAX;
    let mut changed = Vec::new();
    changed.push((0,0));
    while let Some((yy,xx)) = changed.pop() {
        let y = yy;
        let x = xx;

        println!("{} ({},{}) {}", changed.len(), y, x, smalles_so_far);

        for vis in &grid[y][x].visits.clone() {
            add_possible_mov(vis, &mut grid, y, x, &mut changed, &mut smalles_so_far);
        }
    }

    println!("{:?}", smalles_so_far);
    // part1 1004
    // part1 1529
    */
}

/*
fn _add_possible_mov(
    b:&Visit, 
    grid:&mut Vec<Vec<Block>>, 
    y:usize, x:usize, 
    changed:&mut Vec<(usize,usize)>,
    smallest: &mut usize) {
    match b.dir {
        Direction::Up => {
            if b.till_last_turn < MIN {
                insert(grid, &[Direction::Down,Direction::Left,Direction::Right],y,x,b, changed, smallest);
            } else if b.till_last_turn < MAX {
                insert(grid, &[Direction::Down],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Down,Direction::Up],y,x,b, changed, smallest);
            }
        }
        Direction::Down => {
            if b.till_last_turn < MIN {
                insert(grid, &[Direction::Up,Direction::Left,Direction::Right],y,x,b, changed, smallest);
            } else if b.till_last_turn < MAX {
                insert(grid, &[Direction::Up],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Up,Direction::Down],y,x,b, changed, smallest);
            }
        }
        Direction::Right => {
            if b.till_last_turn < MIN {
                insert(grid, &[Direction::Up,Direction::Left,Direction::Down],y,x,b, changed, smallest);
            } else if b.till_last_turn < MAX {
                insert(grid, &[Direction::Left],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Left,Direction::Right],y,x,b, changed, smallest);
            }
        }
        Direction::Left => {
            if b.till_last_turn < MIN {
                insert(grid, &[Direction::Up,Direction::Right,Direction::Down],y,x,b, changed, smallest);
            } else if b.till_last_turn < MAX {
                insert(grid, &[Direction::Right],y,x,b, changed, smallest);
            } else {
                insert(grid, &[Direction::Right,Direction::Left],y,x,b, changed, smallest);
            }
        }
    }
}

fn _insert(
    grid:&mut Vec<Vec<Block>>, 
    banned:&[Direction], 
    y:usize, x:usize, 
    b:&Visit, 
    changed:&mut Vec<(usize,usize)>,
    smallest:&mut usize) {

    let up = if b.dir == Direction::Up { b.till_last_turn + 1 } else { 1 };
    let down = if b.dir == Direction::Down { b.till_last_turn + 1 } else { 1 };
    let left = if b.dir == Direction::Left { b.till_last_turn + 1 } else { 1 };
    let right = if b.dir == Direction::Right { b.till_last_turn + 1 } else { 1 };

    if y < grid.len()-1 && !banned.contains(&Direction::Down) {
        change(b, grid, y+1, x, down, Direction::Down, changed, smallest);
    }
    if x < grid[y].len()-1 && !banned.contains(&Direction::Right) {
        change(b, grid, y, x+1, right, Direction::Right, changed, smallest);
    }

    if x > 0 && !banned.contains(&Direction::Left) {
        change(b, grid, y, x-1, left, Direction::Left, changed, smallest);
    }
    if y > 0 && !banned.contains(&Direction::Up) {
        change(b, grid, y-1, x, up, Direction::Up, changed, smallest);
    }
}

fn _change(
    b:&Visit, 
    grid:&mut Vec<Vec<Block>>, 
    y:usize, x:usize,
    diff:u8, 
    dir:Direction,
    changed:&mut Vec<(usize,usize)>,
    smallest:&mut usize) {

    let Visit{ heat_loss, .. } = b.clone();
    let to_add = Visit{
        dir,
        till_last_turn:diff,
        heat_loss:heat_loss + grid[y][x].heat_absorption,
    };

    if (y,x) == (grid.len()-1, grid[y].len()-1) {
        if diff >= MIN {
            *smallest = min(*smallest, to_add.heat_loss);
        }
        return
    }


    let dy = grid.len()-1 - y;
    let dx = grid[y].len()-1 - x;

    if to_add.heat_loss + dy + dx > *smallest 
        || grid[y][x].visits.iter().any(|e| 
                                    (e.heat_loss <= to_add.heat_loss 
                                     && e.dir == to_add.dir
                                    && e.till_last_turn <= to_add.till_last_turn
                                    && e.till_last_turn >= MIN)) {
        return
    }

    //println!("{} {:?} {:?}", changed.len(), (y,x), to_add);

    if !grid[y][x].visits.contains(&to_add){
        grid[y][x].visits.push(to_add);
        if !changed.contains(&(y,x)) {
            changed.push((y,x));
        }
    }
}
*/

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
