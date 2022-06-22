mod base;
mod solver;
mod formats;

use solver::SimpleSolver;
use solver::Solver;

fn main() {

    let b = base::Block::COLOR(1);

    let mut rows = Vec::new();
    rows.push(vec![(1, b)]);
    rows.push(vec![(1, b),(1, b)]);
    rows.push(vec![(3, b)]);
    rows.push(vec![(4, b)]);
    rows.push(vec![(1, b),(1, b),(1, b)]);
    
    let mut cols = Vec::new();
    cols.push(vec![(1, b),(1, b),(1, b)]);
    cols.push(vec![(3, b)]);
    cols.push(vec![(3, b)]);
    cols.push(vec![(1, b),(1, b)]);
    cols.push(vec![(2, b)]);
    
    let mut grid = base::Grid::new(rows, cols);

    let solver = SimpleSolver::new();
    let _ = solver.solve(&mut grid);
    println!("{:?}", grid);
}
