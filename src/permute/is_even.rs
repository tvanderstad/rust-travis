use crate::matrix::determinant::laplace_expansion;
use crate::matrix::Matrix;
use crate::permute::Permutation;

pub fn is_even(p: &Permutation) -> bool {
    laplace_expansion(&Matrix::from(p)) == 1
}

#[cfg(test)]
mod test {
    use super::is_even;
    use crate::permute::Permutation;
    use crate::permute::test::TEST_LEN_PERMUTATION;

    #[test]
    fn ordered() {
        for i in 0..TEST_LEN_PERMUTATION {
            assert!(is_even(&Permutation::new(i)));
        }
    }

    #[test]
    fn two() {
        assert!(is_even(&Permutation{ v: vec![0, 1] }));
        assert!(!is_even(&Permutation{ v: vec![1, 0] }));
    }

    #[test]
    fn three() {
        assert!(is_even(&Permutation{ v: vec![0, 1, 2] }));
        assert!(!is_even(&Permutation{ v: vec![0, 2, 1] }));
        assert!(is_even(&Permutation{ v: vec![1, 2, 0] }));
        assert!(!is_even(&Permutation{ v: vec![1, 0, 2] }));
        assert!(is_even(&Permutation{ v: vec![2, 0, 1] }));
        assert!(!is_even(&Permutation{ v: vec![2, 1, 0] }));
    }
}
