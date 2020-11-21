use crate::matrix::minor::Minor;
use crate::matrix::Matrix;

pub fn laplace_expansion(m: &impl Matrix) -> i32 {
    if let (2, 2) = m.dimensions() {
        m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0)
    } else {
        let mut total = 0;
        for i in 0..m.dimensions().0 {
            let minor = Minor {
                matrix: m,
                removed_i: vec![i],
                removed_j: vec![0],
            };
            let minor_determinant = laplace_expansion(&minor);
            total += (if i % 2 == 0 { 1 } else { -1 }) * minor_determinant;
        }
        total
    }
}
