use super::base::Grid;
use super::formats::FormatHandler;
use super::formats::makhorin::Makhorin;
use std::path::Path;
use std::fs;
use std::io::Cursor;
use rustache::{HashBuilder, Render};

fn diff_to_string(diff: i32) -> String {
    match diff {
        -1 => "unknown".to_string(),
        d => d.to_string()
    }
}

pub fn write_output(path: &String, i: usize, grid: &Grid, difficulty: i32) -> std::io::Result<()> {
    let d = diff_to_string(difficulty);
    
    let template = HashBuilder::new()
        .insert("rows", grid.rows().to_string())
        .insert("cols", grid.cols().to_string())
        .insert("difficulty", d)
        .insert("index", i.to_string());
    
    let mut rendered = Cursor::new(Vec::new());
    template.render(path, &mut rendered).unwrap();
    let rendered_path = String::from_utf8(rendered.into_inner()).unwrap();

    let p = Path::new(&rendered_path);
    let dirs = p.parent().unwrap();
    fs::create_dir_all(dirs)?;

    let grid_str = Makhorin::stringify(&grid);
    fs::write(p, grid_str)
}

pub fn print_difficulty(grid: &Grid, difficulty: i32) {
    println!("\n{:?}\n{:?}", grid.row_descriptions(), grid.col_descriptions());
    println!("#Difficulty: {}", diff_to_string(difficulty));
}