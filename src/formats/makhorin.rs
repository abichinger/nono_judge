use super::FormatHandler;
use super::parse_desc;

use crate::base::Grid;
use std::io::Result;

pub struct Makhorin {}

impl FormatHandler for Makhorin {
    fn parse(source: &str) -> Result<Grid> {        
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut cols_active = false;
        
        let lines = source.split("\n");
        for mut line in lines {
            line = line.trim();
            if line == "" {
                continue;
            }

            match &line[..1] {
                "#" => continue,
                "*" => continue,
                "" => continue,
                "&" => {
                    cols_active = true;
                    continue
                },
                _ => {}
            };

            let desc = parse_desc(line);
            if let Err(err) = desc {
                return Err(err);
            }

            if cols_active {
                cols.push(desc.unwrap()) 
            } else {
                rows.push(desc.unwrap())
            }
        }

        return Ok(Grid::new(rows, cols))
    }
    
    fn stringify(grid: &Grid) -> String {
        let mut str = String::new();
        for row in grid.row_descriptions() {
            if row.len() == 0 {
                str = format!("{}0\n", str);
                continue;
            }
            let row_str: Vec<_> = row.iter().map(ToString::to_string).collect();
            str = format!("{}{}\n", str, row_str.join(" "));
        }
        str += "&\n";
        for col in grid.col_descriptions() {
            if col.len() == 0 {
                str = format!("{}0\n", str);
                continue;
            }
            let col_str: Vec<_> = col.iter().map(ToString::to_string).collect();
            str = format!("{}{}\n", str, col_str.join(" "));
        }
        return str
    }
}