use crate::matrix::Matrix;

pub struct Minor<'a, M: Matrix> {
    pub matrix: &'a M,
    pub removed_i: Vec<usize>,
    pub removed_j: Vec<usize>,
}

impl<'a, M: Matrix> From<&'a M> for Minor<'a, M> {
    fn from(m: &'a M) -> Self {
        Minor {
            matrix: m,
            removed_i: vec![],
            removed_j: vec![],
        }
    }
}

impl<M: Matrix> Matrix for Minor<'_, M> {
    fn dimensions(&self) -> (usize, usize) {
        let (size_i, size_j) = self.matrix.dimensions();
        (size_i - self.removed_i.len(), size_j - self.removed_j.len())
    }

    fn get(&self, i: usize, j: usize) -> i32 {
        let (size_i, size_j) = self.matrix.dimensions();
        let i = (0..size_i)
            .filter(|i| !self.removed_i.contains(&i))
            .nth(i)
            .unwrap();
        let j = (0..size_j)
            .filter(|j| !self.removed_i.contains(&j))
            .nth(j)
            .unwrap();
        self.matrix.get(i, j)
    }
}
