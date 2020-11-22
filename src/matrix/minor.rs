use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Minor<'a> {
    pub matrix: &'a Matrix,
    pub removed_i: Vec<usize>,
    pub removed_j: Vec<usize>,
}

impl<'a> Minor<'a> {
    pub fn translate_i(&self, i: usize) -> usize {
        (0..self.matrix.dimensions().0)
            .filter(|i| !self.removed_i.contains(&i))
            .nth(i)
            .unwrap()
    }

    pub fn translate_j(&self, j: usize) -> usize {
        (0..self.matrix.dimensions().1)
            .filter(|j| !self.removed_j.contains(&j))
            .nth(j)
            .unwrap()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let (size_i, size_j) = self.matrix.dimensions();
        (size_i - self.removed_i.len(), size_j - self.removed_j.len())
    }

    pub fn get(&self, i: usize, j: usize) -> i32 {
        self.matrix.get(self.translate_i(i), self.translate_j(j))
    }
}

impl<'a> From<&'a Matrix> for Minor<'a> {
    fn from(m: &'a Matrix) -> Self {
        Minor {
            matrix: m,
            removed_i: vec![],
            removed_j: vec![],
        }
    }
}
