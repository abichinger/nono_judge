mod base;

use base::Line;

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

    let r = &rows;
    let c = &cols;
    
    let mut grid = base::Grid::new(r, c);

    println!("L1:");
    
    let l1 = grid.get_row(1);
    for x in l1.iter_candidates() {
        println!("{:?}", x);
    }

    println!("sovle r0:");
    let l0 = grid.get_row(0);
    println!("{:?}", l0.solve());

    println!("sovle r4:");
    let l4 = grid.get_row(4);
    println!("{:?}", l4.solve());

    grid.solve();
    println!("{:?}", grid);
}
