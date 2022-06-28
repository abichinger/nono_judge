#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Block{
    UNKNOWN,
    COLOR(u8)
}

impl Block {
    pub fn merge(&self, b:&Block) -> Self {
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
pub struct Grid {
    row_desc: Vec<Description>,
    col_desc: Vec<Description>,
    blocks: Vec<Vec<Block>>
}

impl Grid {

    pub fn new(rows: Vec<Description>, cols: Vec<Description>) -> Grid {
        let mut blocks = Vec::new();

        for _ in 0..rows.len() {
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

    pub fn get_row(&self, i: usize) -> Row {
        Row {
            grid: self,
            i: i
        }
    }

    pub fn get_col(&self, i: usize) -> Column {
        Column {
            grid: self,
            i: i
        }
    }

    fn parse_desc(&self, descriptions: &Vec<Description>) -> Vec<Vec<usize>> {
        descriptions.iter().map(|desc| {
            desc.iter().map(|(span, _)| *span).collect()
        }).collect()
    }

    pub fn row_descriptions(&self) -> Vec<Vec<usize>> {
        return self.parse_desc(&self.row_desc)
    }

    pub fn col_descriptions(&self) -> Vec<Vec<usize>> {
        return self.parse_desc(&self.col_desc)
    }

    pub fn from_blocks(blocks: &Vec<Vec<Block>>) -> Grid {
        let mut rows: Vec<Description> = Vec::new();
        let mut cols: Vec<Description> = Vec::new();

        let r_len = blocks.len();
        let c_len = blocks[0].len();

        for r in 0..r_len {
            rows.push(Description::from_blocks(&blocks[r]))
        }

        
        for c in 0..c_len {
            let mut col = Vec::new();

            for r in 0..r_len {
                col.push(blocks[r][c])
            }
            cols.push(Description::from_blocks(&col))
        }

        return Grid::new(rows, cols);
    }

    fn _has_empty_line(&self, descriptions: &Vec<Description>) -> bool {
        for i in 0..descriptions.len() {
            if descriptions[i].len() == 0 {
                return true
            }
        }
        return false
    }

    pub fn has_empty_line(&self) -> bool {
        return self._has_empty_line(&self.row_desc) || self._has_empty_line(&self.col_desc)
    }

}

pub struct Row<'a> {
    grid: &'a Grid,
    i: usize
}

pub struct Column<'a> {
    grid: &'a Grid,
    i: usize
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
    fn from_blocks(blocks: &Vec<Block>) -> Description;
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
        let mut line = Vec::new();
        line.resize(line_length, fill);

        for x in self.iter() {
            for _ in 0..x.0 {
                line[i] = x.1;
                i += 1;
            }
        }

        return line
    }

    fn from_blocks(blocks: &Vec<Block>) -> Description {
        let mut desc = Vec::new();
        let mut span: Span = 0;
        let mut cur_color = 0;
        for b in blocks {
            let c = match b {
                Block::UNKNOWN => 0,
                Block::COLOR(x) => *x
            };

            if c != cur_color {
                if cur_color != 0 {
                    desc.push((span, Block::COLOR(cur_color)))
                }
                cur_color = c;
                span = 1;
            } else {
                span+=1;
            }
        }
        if cur_color != 0 {
            desc.push((span, Block::COLOR(cur_color)))
        }
        return desc;
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