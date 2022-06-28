# NonoJudge
*generate and solve nonogram puzzles, which are solvable by logical reasoning*

## Usage

### Build

```
git clone https://github.com/abichinger/nono_judge.git
cd nono_judge && cargo build --release
```

### CLI

```
$ ./target/release/nono_judge --help

NonoJudge can generate and solve nonogram puzzles, which are solvable by logical reasoning

USAGE:
    nono_judge <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    gen      generate nonogram puzzles
    help     Print this message or the help of the given subcommand(s)
    judge    determine difficulty of a nonogram puzzle
```

### Usage Examples

generate all 5x5 puzzles which have at least a diffictuly of 5:
```
nono_judge gen -n 0 --non-empty | nono_judge judge --output out/5x5_all/{{difficulty}}/{{index}}.mak --dgte 5
```

## Motivation

try out Rust

## TODO

- handle all input/output cases (e.g. gen can only output to stdout)
- support multiple formats
- rewrite base.rs