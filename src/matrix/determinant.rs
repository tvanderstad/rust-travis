use crate::matrix::minor::Minor;
use crate::matrix::Matrix;

pub fn laplace_expansion(m: &Matrix) -> i32 {
    laplace_expansion_minor(&mut Minor {
        matrix: m,
        removed_i: vec![],
        removed_j: vec![],
    })
}

pub fn laplace_expansion_minor(m: &mut Minor) -> i32 {
    match m.dimensions() {
        (0, 0) => 1,
        (1, 1) => m.get(0, 0),
        _ => {
            let mut total = 0;
            for i in 0..m.dimensions().0 {
                let e = m.get(i, 0);
                m.removed_i.push(m.translate_i(i));
                m.removed_j.push(m.translate_j(0));
                let minor_determinant = laplace_expansion_minor(m);
                m.removed_i.pop();
                m.removed_j.pop();
                total += (if i % 2 == 0 { 1 } else { -1 }) * e * minor_determinant;
            }
            total
        }
    }
}

#[cfg(test)]
mod test {
    use super::laplace_expansion;
    use crate::matrix::Matrix;

    #[test]
    fn empty() {
        let matrix = Matrix { m: vec![] };
        assert_eq!(1, laplace_expansion(&matrix));
    }

    #[test]
    fn one_by_one() {
        let matrix = Matrix { m: vec![vec![0]] };
        assert_eq!(0, laplace_expansion(&matrix));
    }

    #[test]
    fn two_by_two_diagonal() {
        let matrix = Matrix { m: vec![
            vec![1, 0],
            vec![0, 1],
        ] };
        assert_eq!(1, laplace_expansion(&matrix));
    }

    #[test]
    fn two_by_two_diagonal_reverse() {
        let matrix = Matrix { m: vec![
            vec![0, 1],
            vec![1, 0],
        ] };
        assert_eq!(-1, laplace_expansion(&matrix));
    }

    #[test]
    fn three_by_three_diagonal() {
        let matrix = Matrix { m: vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ] };
        assert_eq!(1, laplace_expansion(&matrix));
    }

    #[test]
    fn three_by_three_diagonal_reverse() {
        let matrix = Matrix { m: vec![
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![1, 0, 0],
        ] };
        assert_eq!(-1, laplace_expansion(&matrix));
    }

    #[test]
    fn three_by_three_singular() {
        let matrix = Matrix { m: vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ] };
        assert_eq!(0, laplace_expansion(&matrix));
    }

    #[test]
    fn four_by_four_diagonal() {
        let matrix = Matrix { m: vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ] };
        assert_eq!(1, laplace_expansion(&matrix));
    }

    #[test]
    fn four_by_four_singular() {
        let matrix = Matrix { m: vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ] };
        assert_eq!(0, laplace_expansion(&matrix));
    }
}
