use crate::matrix;
use crate::permute::Permutation;

impl From<&Permutation> for matrix::Dense {
    fn from(p: &Permutation) -> Self {
        let mut m = matrix::Dense::new(p.v.len());
        for (i, x) in p.v.iter().enumerate() {
            m.m[i.clone()][x.clone()] = 1;
        }
        m
    }
}
