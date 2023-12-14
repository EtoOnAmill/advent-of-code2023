use std::fs;

fn gridify(s:&str) -> Vec<Vec<char>> {
    s
        .split("\n")
        .filter_map(|e| if e.len() != 0 {Some(e.chars().collect())} else {None})
        .collect()
}

fn column_simmetry(grid:&Vec<Vec<char>>) -> Vec<usize> {
    let mut poss_symms = Vec::new();
    for i in 0..(grid[0].len()-1) {

        let mut symmetric = true;
        for y in grid {
            symmetric = symmetric && y[i] == y[i+1];
        }

        if symmetric {
            poss_symms.push(i);
        }
    }

    let mut true_symms = Vec::new();
    for i in poss_symms {
        let before = (0..=i).rev();
        let after = (i+1)..grid[0].len();
        let zipped = before.zip(after);

        let mut symm = true;
        for (b,a) in zipped {
            for y in grid {
                symm = symm && y[b] == y[a];
            }
        }
        
        if symm { true_symms.push(i+1) }
    }
    true_symms
}

fn row_simmetry(grid:&Vec<Vec<char>>) -> Vec<usize> {
    let mut poss_symms = Vec::new();
    for n in 0..(grid.len()-1) {
        if grid[n] == grid[n+1] { poss_symms.push(n) }
    }
    
    let mut true_symms = Vec::new();
    for i in poss_symms {
        let before = (0..=i).rev();
        let after = (i+1)..grid.len();
        let zipped = before.zip(after);

        let mut simmetry = true;
        for (b,a) in zipped {
            if grid[b] != grid[a] { simmetry = false; break } 
        }
        if simmetry { true_symms.push(i+1) }
    }

    true_symms
}

fn correct_smidge(grid:&Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<char>>{
    let mut ret = grid.clone();
    ret[y][x] = match ret[y][x] {
        '#' => '.',
        '.' => '#',
        _ => panic!()
    };
    ret
}

fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/13day/input").unwrap();
    let test = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
    let sections = input
        .split("\n\n")
        .filter(|e| e.len() != 0);

    let mut tot = 0;
    for sect in sections {
        let smudged_grid = gridify(sect);

        let old_row_syms = row_simmetry(&smudged_grid);
        let old_column_syms = column_simmetry(&smudged_grid);

        let mut fixed = false;
        for x in 0..smudged_grid[0].len(){
            for y in 0..smudged_grid.len(){
                let grid = correct_smidge(&smudged_grid, x, y);

                let row_syms = row_simmetry(&grid);
                let column_syms = column_simmetry(&grid);
                if old_column_syms == column_syms && old_row_syms == row_syms { continue }
                
                let row_tot = 100*row_syms
                    .iter()
                    .filter(|e| !old_row_syms.contains(e))
                    .fold(0, |acc,e| acc+e);
                let column_tot = column_syms
                    .iter()
                    .filter(|e| !old_column_syms.contains(e))
                    .fold(0, |acc,e| acc+e);

                tot += row_tot + column_tot;

                if row_syms.len() != 0 || column_syms.len() != 0 { 
                    fixed = true; 
                    println!("row: {:?}\ncolumn: {:?}", row_syms, column_syms);
                    println!("{}\n{tot}\n", grid.iter().map(|e| e.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
                    break 
                }
            }
            if fixed { break }
        }
    }
    println!("{}", tot);
}
