use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Block{
    UNKNOWN,
    COLOR(u8)
}

impl Block {
    fn merge(&self, b:&Block) -> Self {
        match (self, b) {
            (Block::COLOR(c1), Block::COLOR(c2)) => {
               if c1 == c2 { self.clone() } else { Block::UNKNOWN }
            },
            (Block::UNKNOWN, _) => Block::UNKNOWN,
            (_, Block::UNKNOWN) => Block::UNKNOWN,
        }
    }
}

type Span = usize;
type LineDescription = Vec<(Span, Block)>;
pub type Description = Vec<(Span, Block)>;
//type DescriptionSlice<'a> = &'a [(Span, Block)];

#[derive(Debug)]
pub struct Grid<'a> {
    row_desc: &'a[Description],
    col_desc: &'a[Description],
    blocks: Vec<Vec<Block>>
}

impl<'a> Grid<'a> {

    pub fn new(rows: &'a[Description], cols: &'a[Description]) -> Grid<'a> {
        let mut blocks = Vec::new();

        for _ in rows {
            let vec = vec![Block::UNKNOWN; cols.len()];
            blocks.push(vec)
        }
    
        Grid {
            row_desc: rows,
            col_desc: cols,
            blocks: blocks,
        }
    }
    
    pub fn rows(&self) -> usize {
        self.row_desc.len()
    }

    pub fn cols(&self) -> usize {
        self.col_desc.len()
    }

    pub fn get(&self, row: usize, col: usize) -> Block {
        self.blocks[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, b: Block) {
        self.blocks[row][col] = b
    } 

    pub fn get_row(&'a self, i: usize) -> Row {
        Row {
            grid: self,
            i: i
        }
    }

    pub fn get_col(&'a self, i: usize) -> Column {
        Column {
            grid: self,
            i: i
        }
    }

    pub fn solve(&mut self) {

        let mut solved_rows = HashSet::new();
        let mut solved_cols = HashSet::new();
        let mut progress;

        while solved_rows.len() != self.rows() || solved_cols.len() != self.cols() {
            progress = false;
            
            //solve rows
            for i in 0..self.rows() {
                if solved_rows.contains(&i) {
                    continue
                }
                let new_blocks = self.get_row(i).solve();
                if new_blocks.len() > 0 {
                    progress = true;
                }
                for (col, b) in new_blocks {
                    self.set(i, col, b);
                }
                if self.get_row(i).solved() {
                    solved_rows.insert(i);
                }
            }

            //solve columns
            for i in 0..self.cols() {
                if solved_cols.contains(&i) {
                    continue
                }
                let new_blocks = self.get_col(i).solve();
                if new_blocks.len() > 0 {
                    progress = true;
                }
                for (row, b) in new_blocks {
                    self.set(row, i, b);
                }
                if self.get_col(i).solved() {
                    solved_cols.insert(i);
                }
            }

            if progress == false {
                panic!("unable to solve");
            }
        }
    }
}

pub struct Row<'a> {
    grid: &'a Grid<'a>,
    i: usize
}

pub struct Column<'a> {
    grid: &'a Grid<'a>,
    i: usize
}

fn merge(l1: &mut [Block], l2: &[Block]) {
    for (i, b) in l2.iter().enumerate() {
        l1[i] = l1[i].merge(b);
    }
}

pub trait Line {
    fn is_solved(&self, i: usize) -> bool {
        self.get(i) != Block::UNKNOWN
    }

    fn solved(&self) -> bool {
        for i in 0..self.len() {
            if !self.is_solved(i) {
                return false
            }
        }
        return true
    }
    
    fn solve(&self) -> Vec<(usize, Block)>{
        let mut vec = Vec::new();
        
        if self.solved() {
            return vec
        }

        let mut iter = self.iter_candidates();
        let first = iter.next();

        if first == None {
            return vec
        }
        let mut line = first.unwrap();

        while let Some(l) = iter.next() {
            merge(&mut line, &l)
        }

        for i in 0..self.len() {
            if !self.is_solved(i) && line[i] != Block::UNKNOWN {
                vec.push((i, line[i]))
            }
        }

        return vec
    }

    fn len(&self) -> usize;
    fn min_len(&self) -> usize;
    fn get(&self, i: usize) -> Block;
    fn get_desc(&self) -> &Description;

    fn known_blocks(&self) -> Vec<(usize, Block)> {
        let mut vec = Vec::new();
        for i in 0..self.len() {
            if self.is_solved(i) {
                vec.push((i, self.get(i)))
            }
        }
        return vec
    }

    fn iter(&self) -> DescIterator {
        let mut res = Vec::new();

        for (i, item) in self.get_desc().iter().enumerate() {
            res.push((if i==0 {0} else {1}, Block::COLOR(0)));
            res.push(*item);
        }
        
        let iter = DescIterator {
            line_length: self.len()+1,
            desc_length: self.get_desc().len(),
            length: res.length(),
            res: res
        };

        return iter
    }

    fn iter_candidates(&self) -> CandidateIterator {
        CandidateIterator {
            iter: self.iter(),
            known_blocks: self.known_blocks()
        }
    }
}

impl<'a> Line for Row<'a> {

    fn len(&self) -> usize {
        self.grid.col_desc.len()
    }

    fn min_len(&self) -> usize {
        self.get_desc().length() + self.get_desc().len() - 1
    }

    fn get(&self, i: usize) -> Block {
        self.grid.get(self.i, i)
    }

    fn get_desc(&self) -> &Description {
        &self.grid.row_desc[self.i]
    }
}

impl<'a> Line for Column<'a> {

    fn len(&self) -> usize {
        self.grid.row_desc.len()
    }

    fn min_len(&self) -> usize {
        self.get_desc().length() + self.get_desc().len() - 1
    }

    fn get(&self, i: usize) -> Block {
        self.grid.get(i, self.i)
    }

    fn get_desc(&self) -> &Description {
        &self.grid.col_desc[self.i]
    }
}

pub trait DescriptionTrait {
    fn length(&self) -> usize;
    fn to_line(&self, line_length: usize, fill: Block) -> Vec<Block>;
}

impl DescriptionTrait for Description {
    fn length(&self) -> usize {
        if self.len() == 0 {
            return 0;
        }
        let mut len = 0;
        for x in self.iter() {
            len += x.0
        }
        return len
    }

    fn to_line(&self, line_length: usize, fill: Block) -> Vec<Block> {
        let mut i = 0;
        let mut line = Vec::with_capacity(line_length);
        line.resize(5, fill);

        for x in self.iter() {
            for _ in 0..x.0 {
                line[i] = x.1;
                i += 1;
            }
        }

        return line
    }
}

pub struct DescIterator {
    line_length: usize,
    desc_length: usize,
    length: usize,
    res: Description
}

impl Iterator for DescIterator {
    type Item = LineDescription;
    
    fn next(&mut self) -> Option<Self::Item> {

        let len_left = self.line_length - self.length;
        if len_left <= 0 {
            for i in 0..(self.desc_length-1) {
                let j = i*2;
                if self.res[j].0 > 1 {
                    self.length = self.length - self.res[j].0 + 2;
                    self.res[j].0 = 1;
                    self.res[j+2].0 += 1;

                    let mut res = self.res.clone();
                    res[0].0 = res[0].0 - 1;
                    return Some(res)
                }
            }
            return None
        }
        else {
            self.res[0].0 += 1;
            self.length += 1;
        }

        let mut res = self.res.clone();
        res[0].0 = res[0].0 - 1;
        return Some(res)
    }
}

pub struct CandidateIterator {
    iter: DescIterator,
    known_blocks: Vec<(usize, Block)>
}

impl Iterator for CandidateIterator {
    type Item = Vec<Block>;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(desc) = self.iter.next() {

            let line = desc.to_line(self.iter.line_length, Block::COLOR(0));
            let mut is_candidate = true;

            for x in self.known_blocks.iter() {
                if line[x.0] != x.1 {
                    is_candidate = false;
                    break;
                } 
            }

            if is_candidate {
                return Some(line);
            }
        }
        return None
    }
}