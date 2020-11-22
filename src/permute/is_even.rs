use crate::matrix::determinant::laplace_expansion;
use crate::matrix::Matrix;
use crate::permute::Permutation;

pub fn is_even(p: &Permutation) -> bool {
    laplace_expansion(&Matrix::from(p)) == 1
}
