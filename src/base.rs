
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Block{
    UNKNOWN,
    COLOR(u8)
}

impl Block {
    fn fix_to(&self, b:&Block) -> Self {
        match (self, b) {
            (Block::COLOR(c1), Block::COLOR(c2)) => {
               if c1 == c2 { self.clone() } else { Block::UNKNOWN }
            },
            (Block::UNKNOWN, _) => b.clone(),
            (_, Block::UNKNOWN) => self.clone(),
        }
    }
}

type Span = usize;
type LineDescription = Vec<(Span, Block)>;
pub type Description = Vec<(Span, Block)>;
//type DescriptionSlice<'a> = &'a [(Span, Block)];

pub struct Grid {
    row_desc: Vec<Description>,
    col_desc: Vec<Description>,
    blocks: Vec<Vec<Block>>
}

impl Grid {
    fn new(rows: Vec<Description>, cols: Vec<Description>) -> Self {
        let blocks = Vec::new();

        for row in rows {
            let vec = vec![Block::UNKNOWN; cols.len()];
            blocks.push(vec)
        }

        return Grid {
            row_desc: rows,
            col_desc: cols,
            blocks: blocks,
        }
    }
    
    fn rows(&self) -> usize {
        self.row_desc.len()
    }

    fn cols(&self) -> usize {
        self.col_desc.len()
    }

    fn get(&self, row: usize, col: usize) -> Block {
        self.blocks[row][col]
    }

    fn set(&self, row: usize, col: usize, b: Block) {
        self.blocks[row][col] = b
    } 

    fn get_row(&self, i: usize) -> Line {

    }

    fn get_col(&self, i: usize) -> Line {

    }
}

pub struct Row<'a> {
    grid: &'a Grid,
    i: usize
}

pub struct Column {
    grid: &'a Grid,
    i: usize
}

pub trait Line {

    fn solved_amount(&self) -> usize {
        self.blocks.iter().filter(|&&b| b != Block::UNKNOWN).count()
    }

    fn len(&self) {

    }

    fn min_len(&self) -> usize {

    }

    fn get(&self, i: usize) -> Block {

    }

    fn set(&self, i: usize, b: Block) {

    }

    fn get_desc(&self) -> Description {

    }
}

pub trait Length {
    fn length(&self) -> usize;
}

impl Length for Line {
    fn length(&self) -> usize {
        self.blocks.len()
    }
}

impl Length for Description {
    fn length(&self) -> usize {
        if self.len() == 0 {
            return 0;
        }
        let mut len = 0;
        for x in self.iter() {
            len += x.0
        }
        return len - 1
    }
}

pub struct LineIterator {
    length: usize,
    desc_length: usize,
    res: Description
}

impl From<Line> for LineIterator {
    fn from(line: Line) -> Self {
        let mut iter = LineIterator {
            length: line.blocks.len() +1,
            desc_length: line.desc.len(),
            res: Vec::new()
        };

        for (i, item) in line.desc.iter().enumerate() {
            iter.res.push((if i==0 {0} else {1}, Block::COLOR(0)));
            iter.res.push(*item);
        }

        return iter
    }
}

impl Iterator for LineIterator {
    type Item = LineDescription;
    
    fn next(&mut self) -> Option<Self::Item> {

        let len_left = self.length - self.res.length();
        if len_left <= 0 {
            for i in 0..(self.desc_length-1) {
                let j = i*2;
                if self.res[j].0 > 1 {
                    self.res[j].0 = 1;
                    self.res[j+2].0 += 1;
                    return Some(self.res.clone())
                }
            }
            return None
        }
        else {
            self.res[0].0 += 1
        }        

        return Some(self.res.clone())
    }
}


/*fn possibilities(line: Line, lineIndex: usize, descIndex: usize, res: Description, fill: Block){

    let desc: DescriptionSlice = &line.desc[descIndex..];
    let len = desc.length() + 1;
    let len_left = line.blocks.len() - lineIndex;

    for i in 1..(len_left - len) {

        res.push((i, fill));
        res.push(line.desc[descIndex]);

        if descIndex >= line.desc.len()-1 {
            //yield res
        } 
        else {
            possibilities(line, lineIndex + i + line.desc[descIndex].0, descIndex+1, res, fill);
        }

        res.pop();
        res.pop();
    }
}

pub struct Description {
    pub span: Vec<usize>,
    pub color: Vec<usize>,
    i: usize
}

impl Iterator for Description {
    type Item = Vec<Block>;
    
    fn next(&mut self) -> Option<Self::Item> {
        //???????
        Some(Vec::new())
    }
}

pub trait DescriptionTrait {
    fn get_possibilities(self) -> Vec<LineDescription>;
    fn remaining_length(self, i:usize) -> usize;
}

impl IntoIterator for Description {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.span.into_iter()
    }
}

fn possibilities(desc:Description, line:&LineDescription, vec:&Vec<LineDescription>, i:usize) {
    let rlen = desc.remaining_length(i);
    
}

impl DescriptionTrait for Description {

    fn remaining_length(self, i:usize) -> usize{
        let mut remaining = Description {
            span: &self.span[1..],
            color: &self.color[1..]
        };
        remaining.vec.copy_from_slice(&self.vec[1..]);
        remaining.length()
    }
}*/

