pub mod determinant;
pub mod minor;

pub trait Matrix {
    fn dimensions(&self) -> (usize, usize);
    fn get(&self, i: usize, j: usize) -> i32;
}

pub struct Dense {
    pub m: Vec<Vec<i32>>,
}

impl Dense {
    pub fn new(len: usize) -> Dense {
        Dense {
            m: vec![vec![0; len]; len],
        }
    }
}

impl Matrix for Dense {
    fn dimensions(&self) -> (usize, usize) {
        (self.m.len(), self.m.first().unwrap().len())
    }

    fn get(&self, i: usize, j: usize) -> i32 {
        self.m[i][j]
    }
}
