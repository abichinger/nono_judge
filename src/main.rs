mod base;

use base::Line;

fn main() {

    let b = base::Block::COLOR(1);

    let mut rows = Vec::new();
    rows.push(vec![(5, b)]);
    rows.push(vec![(1, b),(1, b)]);
    rows.push(vec![(1, b),(1, b)]);
    rows.push(vec![(1, b),(1, b)]);
    rows.push(vec![(1, b),(2, b)]);
    
    let mut cols = Vec::new();
    cols.push(vec![(1, b)]);
    cols.push(vec![(5, b)]);
    cols.push(vec![(1, b)]);
    cols.push(vec![(5, b)]);
    cols.push(vec![(1, b),(1, b)]);

    let r = &rows;
    let c = &cols;
    
    let mut grid = base::Grid::new(r, c);
    
    println!("L1:");

    let l0 = grid.get_row(1);
    for x in l0.iter() {
        println!("{:?}", x);
    }

    grid.set(1,0,base::Block::COLOR(1));

    println!("L1:");
    
    let l1 = grid.get_row(1);
    for x in l1.iter_candidates() {
        println!("{:?}", x);
    }

    
}
