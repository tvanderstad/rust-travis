use crate::matrix::Matrix;
use crate::permute::Permutation;

impl From<&Permutation> for Matrix {
    fn from(p: &Permutation) -> Self {
        let mut m = Matrix::new(p.v.len());
        for (i, x) in p.v.iter().enumerate() {
            m.m[i.clone()][x.clone()] = 1;
        }
        m
    }
}
