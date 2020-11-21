use crate::permute::Permutation;

// based on https://www.cut-the-knot.org/Curriculum/Combinatorics/JohnsonTrotter.shtml
#[derive(Debug)]
pub struct SteinhausJohnsonTrotter {
    pub permutation: Option<Permutation>,
    pub directions: Vec<bool>,
}

impl SteinhausJohnsonTrotter {
    // "Initialize the first permutation with <1 <2 ... <n"
    pub fn new(len: usize) -> Self {
        SteinhausJohnsonTrotter {
            permutation: Some(Permutation::new(len)),
            directions: vec![false; len],
        }
    }
}

impl Iterator for SteinhausJohnsonTrotter {
    type Item = Permutation;

    fn next(&mut self) -> Option<Permutation> {
        let result = self.permutation.clone();

        // "while there exists a mobile integer... find the largest mobile integer k"
        if let Some(ref mut permutation) = self.permutation {
            let max_mobile = permutation
                .v
                .iter()
                .enumerate()
                .filter({
                    let p = &permutation;
                    let directions = &self.directions;
                    move |(i, _)| is_mobile(p, directions, *i)
                })
                .max_by_key(|(_, &k)| k);
            if let Some((i, &k)) = max_mobile {
                // "swap k and the adjacent integer it is looking at"
                let j = if self.directions[i] { i + 1 } else { i - 1 };
                permutation.v.swap(i, j);
                self.directions.swap(i, j);

                // "reverse the direction of all integers larger than k"
                for (i, _) in permutation.v.iter().enumerate().filter(|(_, &x)| x > k) {
                    self.directions[i] ^= true
                }
            } else {
                self.permutation = None;
            }
        }

        result
    }
}

// "a directed integer is said to be mobile if it is greater than its immediate neighbor in the direction it is looking at"
fn is_mobile(permutation: &Permutation, directions: &[bool], i: usize) -> bool {
    if directions[i] {
        if i == permutation.v.len() - 1 {
            false
        } else {
            permutation.v[i] > permutation.v[i + 1]
        }
    } else {
        if i == 0 {
            false
        } else {
            permutation.v[i] > permutation.v[i - 1]
        }
    }
}
