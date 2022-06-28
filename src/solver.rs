use super::base::Grid;
use super::base::Line;
use super::base::Block;
use std::collections::HashSet;

pub trait Solver {
    fn solve(&self, grid: &mut Grid) -> i32;
} 

pub struct SimpleSolver  {
}

impl SimpleSolver {
    pub fn new() -> SimpleSolver {
        SimpleSolver{}
    }
}

impl Solver for SimpleSolver {
    fn solve(&self, grid: &mut Grid) -> i32 {
        let mut solved_rows = HashSet::new();
        let mut solved_cols = HashSet::new();
        let mut progress;
        let mut sweeps: i32 = 0;

        while solved_rows.len() != grid.rows() || solved_cols.len() != grid.cols() {
            progress = false;
            
            //solve rows
            for i in 0..grid.rows() {
                if solved_rows.contains(&i) {
                    continue
                }
                let new_blocks = resolveable(&grid.get_row(i));
                if new_blocks.len() > 0 {
                    progress = true;
                }
                for (col, b) in new_blocks {
                    grid.set(i, col, b);
                }
                if grid.get_row(i).solved() {
                    solved_rows.insert(i);
                    progress = true;
                }
            }

            //solve columns
            for i in 0..grid.cols() {
                if solved_cols.contains(&i) {
                    continue
                }
                let new_blocks = resolveable(&grid.get_col(i)); 
                if new_blocks.len() > 0 {
                    progress = true;
                }
                for (row, b) in new_blocks {
                    grid.set(row, i, b);
                }
                if grid.get_col(i).solved() {
                    solved_cols.insert(i);
                    progress = true;
                }
            }

            sweeps+=1;

            if progress == false {
                return -1;
            }
        }
        return sweeps
    }
}

fn merge(l1: &mut [Block], l2: &[Block]) {
    for (i, b) in l2.iter().enumerate() {
        l1[i] = l1[i].merge(b);
    }
}

fn resolveable(line: &dyn Line) -> Vec<(usize, Block)>{
    let mut vec = Vec::new();
    
    if line.get_desc().len() == 0 {
        for i in 0..line.len() {
            vec.push((i, Block::COLOR(0)))
        }
        return vec
    }

    if line.solved() {
        return vec
    }

    let mut iter = line.iter_candidates();
    let first = iter.next();

    if first == None {
        return vec
    }
    let mut candidate = first.unwrap();

    while let Some(l) = iter.next() {
        merge(&mut candidate, &l)
    }

    for i in 0..line.len() {
        if !line.is_solved(i) && candidate[i] != Block::UNKNOWN {
            vec.push((i, candidate[i]))
        }
    }

    return vec
}