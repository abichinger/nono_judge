pub mod makhorin;

use super::base::Grid;
use super::base::Block;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use makhorin::Makhorin;

pub trait FormatHandler {
    fn parse(source: &str) -> Result<Grid>;
    fn stringify(grid: &Grid) -> String;
}

pub fn parse_desc(str_desc: &str) -> Result<Vec<(usize, Block)>> {
    if str_desc == "0" {
        return Ok(Vec::new())
    }
    
    let b = Block::COLOR(1);
    let mut desc = vec![];
    let str_nums = str_desc.split(" ");
    
    for str_num in str_nums {
        let parsed = str_num.parse::<i32>();
        let n = match parsed {
            Ok(num) => num as usize,
            Err(_) => return Err(Error::new(ErrorKind::Other, "parsing error"))
        };
        desc.push((n, b))
    }
    return Ok(desc);
}

pub fn parse(path: &str) -> Result<Grid> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if path.ends_with("makhorin") {
        return Makhorin::parse(&contents)
    }

    return Err(Error::new(ErrorKind::Other, "unsupported format"))
}