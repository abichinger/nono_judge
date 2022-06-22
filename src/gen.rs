use super::base::Grid;
use super::base::Block;

pub struct GridGenerator {
    done: bool,
    step: u64,
    max: u64,
    rows: Vec<u64>,
    columns: u8,
    blocks: Vec<Vec<Block>>
}

impl GridGenerator {

    pub fn new(width: u8, height: usize, step: Option<u64>, seed: Option<&str>) -> GridGenerator {
        
        let mut blocks = Vec::new();
        for i in 0..height {
            blocks.push(vec![Block::COLOR(0); width.into()])
        }

        let mut gen = GridGenerator {
            step: step.unwrap_or(1),
            max: (2 as u64).pow(width.into()),
            rows: vec![0; height],
            columns: width,
            blocks: blocks,
            done: false
        };

        if let Some(str_seed) = seed {
            gen.set(str_seed);
        }

        return gen
    }

    pub fn add(&mut self, index: usize, n: u64) -> (u64, bool) {
        self.rows[index] += n;
        let carry = self.rows[index] / self.max;
        if carry > 0 {
            if index == self.rows.len()-1 {
                return (0, true)
            }
            self.rows[index] = self.rows[index] % self.max;
        }

        //update blocks
        for j in 0..self.columns {
            self.blocks[index][usize::from(j)] = match self.rows[index] & (1 << j) {
                0 => Block::COLOR(0),
                _ => Block::COLOR(1)
            }
        }

        return (carry, false)
    }

    pub fn set(&mut self, seed: &str) {
        let seeds = seed.split("-");
        for (i, s) in seeds.enumerate() {
            let n = u64::from_str_radix(s, 16);
            self.add(i, n.unwrap());
        }
    }

}

impl Iterator for GridGenerator {
    type Item = Grid;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None
        }
        
        let grid = Grid::from_blocks(&self.blocks);

        let mut overflow: bool;
        let mut carry = self.step;
        let mut i = 0;
        while carry > 0 {
            let add_res = self.add(i, carry);
            carry = add_res.0;
            overflow = add_res.1;
            if overflow {
                self.done = true;
                break
            }
            i += 1;
        }

        return Some(grid)
    }
}