use crate::matrix;
use crate::matrix::determinant::laplace_expansion;
use crate::permute::Permutation;

fn is_even(p: &Permutation) -> bool {
    laplace_expansion(&matrix::Dense::from(p)) == 1
}
