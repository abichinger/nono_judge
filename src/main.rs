mod base;
mod solver;
mod formats;
mod gen;
mod util;

#[cfg(test)]
mod tests;

use clap::{command, arg, value_parser, Command, ArgAction, Arg};
use gen::GridGenerator;
use formats::FormatHandler;
use formats::makhorin::Makhorin;
use std::io;
use solver::SimpleSolver;
use solver::Solver;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("gen")
                .about("generate nonogram puzzles")
                .arg(arg!(-o --output <OUT_PATH> "output path")
                    .required(false)
                    .default_value("stdout")
                )
                .arg(arg!(-r --rows <ROWS> "number of rows")
                    .required(false)
                    .default_value("5")
                    .value_parser(value_parser!(u8))
                )
                .arg(arg!(-c --columns <COLUMNS> "number of columns")
                    .required(false)
                    .default_value("5")
                    .value_parser(value_parser!(u8))
                )
                .arg(arg!(--seed <SEED> "the seed consits of hexadecimal segments seperated by '-'")
                    .required(false)
                    .default_value("0")
                )
                .arg(arg!(--step <STEP> "the step between two puzzles")
                    .required(false)
                    .default_value("1")
                    .value_parser(value_parser!(u64))
                )
                .arg(arg!(-n --number <NUMBER> "number of puzzles to generate")
                    .required(false)
                    .default_value("1")
                    .value_parser(value_parser!(u64))
                )
                .arg(Arg::new("non-empty")
                    .long("--non-empty")
                    .help("skip puzzles with empty lines")
                    .required(false)
                    .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("judge")
                .about("determine difficulty of a nonogram puzzle")
                .arg(arg!(-i --input <IN_PATH> "input path")
                    .required(false)
                    .default_value("stdin")
                )
                .arg(arg!(-o --output <OUT_PATH> "output path - accepts a mustache template, parameters: rows, cols, difficulty, index")
                    .required(false)
                    .default_value("stdout")
                )
                .arg(arg!(--dgte <NUMBER> "minimum difficulty, zero disables the filter")
                    .required(false)
                    .default_value("0")
                    .value_parser(value_parser!(i32))
                )
                .arg(arg!(--dlte <NUMBER> "maximum difficulty, zero disables the filter")
                    .required(false)
                    .default_value("0")
                    .value_parser(value_parser!(i32))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("gen", args)) => {
            let r = *args.get_one::<u8>("rows").unwrap();
            let c = *args.get_one::<u8>("columns").unwrap();
            let n = *args.get_one::<u64>("number").unwrap();
            let seed = args.get_one::<String>("seed").unwrap();
            let step = *args.get_one::<u64>("step").unwrap();
            let non_empty = *args.get_one::<bool>("non-empty").unwrap();

            let gen = GridGenerator::new(c, r, Some(step), Some(seed));
            let mut i = 0;
            for grid in gen {
                if non_empty && grid.has_empty_line() {
                    continue
                }
                let out_str = Makhorin::stringify(&grid);
                println!("{}", out_str);

                i+=1;
                if n != 0 && i == n {
                    break;
                }
            }
        },
        Some(("judge", args)) => {
            let input = args.get_one::<String>("input").unwrap();
            let output = args.get_one::<String>("output").unwrap();
            let min_diff = *args.get_one::<i32>("dgte").unwrap();
            let max_diff = *args.get_one::<i32>("dlte").unwrap();

            let solver = SimpleSolver::new();
            let stdin = io::stdin();
            let buf = &mut String::new();
            let grid_str = &mut String::new();

            let mut i: usize = 0;
            loop {
                buf.clear();
                let bytes = stdin.read_line(buf).expect("read line error");
                if bytes == 0 {
                    break;
                }

                grid_str.push_str(buf);

                if buf == "\n" {
                    let grid_res = Makhorin::parse(grid_str);
                    let mut grid = grid_res.expect("parsing error");
                    grid_str.clear();

                    let difficulty = solver.solve(&mut grid);
                    let index = i;

                    if min_diff != 0 && difficulty < min_diff {
                        continue
                    }

                    if max_diff != 0 && difficulty > max_diff {
                        continue
                    }

                    if output == "stdout" {
                        util::print_difficulty(&grid, difficulty);
                    } else {
                        util::write_output(output, index, &grid, difficulty).expect("write_output");
                    }

                    i+=1;
                }

            }

            
        }
        _ => unreachable!("Exhausted list of subcommands"),
    }
}