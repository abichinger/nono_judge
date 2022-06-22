use super::formats;
use super::formats::FormatHandler;
use super::formats::makhorin::Makhorin;
use super::solver::SimpleSolver;
use super::solver::Solver;
use super::gen::GridGenerator;

#[test]
fn test_simple_solver() {
    
    let mut grid =  match formats::parse("examples/5x5/01.makhorin") {
        Ok(g) => g,
        Err(err) => panic!("{}", err)
    };

    let solver = SimpleSolver::new();
    let res = match solver.solve(&mut grid) {
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };

    assert_eq!(res, ());
}

#[test]
fn test_generator() {

    let mut gen = GridGenerator::new(4,4,Option::None, Option::Some("F-A-1-8"));
    let puzzle_str = Makhorin::stringify(&gen.next().unwrap());
    println!("{}", puzzle_str);

    println!("\n\n{}", Makhorin::stringify(&gen.next().unwrap()));
    println!("\n\n{}", Makhorin::stringify(&gen.next().unwrap()));
}