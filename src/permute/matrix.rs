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

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use crate::permute::Permutation;

    #[test]
    fn empty() {
        let matrix = Matrix { m: vec![] };
        assert_eq!(matrix, Matrix::from(&Permutation::new(0)));
    }

    #[test]
    fn single() {
        let matrix = Matrix { m: vec![vec![1]] };
        assert_eq!(matrix, Matrix::from(&Permutation::new(1)));
    }

    #[test]
    fn two_diagonal() {
        let matrix = Matrix {
            m: vec![vec![1, 0], vec![0, 1]],
        };
        assert_eq!(matrix, Matrix::from(&Permutation::new(2)));
    }

    #[test]
    fn two_reverse() {
        let matrix = Matrix {
            m: vec![vec![0, 1], vec![1, 0]],
        };
        assert_eq!(matrix, Matrix::from(&Permutation{ v: vec![1, 0] }));
    }

    #[test]
    fn three_diagonal() {
        let matrix = Matrix {
            m: vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
        };
        assert_eq!(matrix, Matrix::from(&Permutation::new(3)));
    }

    #[test]
    fn three_reverse() {
        let matrix = Matrix {
            m: vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]],
        };
        assert_eq!(matrix, Matrix::from(&Permutation{ v: vec![2, 1, 0] }));
    }

    #[test]
    fn three_misc() {
        let matrix = Matrix {
            m: vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]],
        };
        assert_eq!(matrix, Matrix::from(&Permutation{ v: vec![2, 0, 1] }));
    }
}
