pub mod determinant;
pub mod minor;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub m: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(len: usize) -> Matrix {
        Matrix {
            m: vec![vec![0; len]; len],
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (
            self.m.len(),
            if self.m.is_empty() {
                0
            } else {
                self.m.first().unwrap().len()
            },
        )
    }

    fn get(&self, i: usize, j: usize) -> i32 {
        self.m[i][j]
    }
}
