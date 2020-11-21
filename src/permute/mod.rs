pub mod is_even;
pub mod matrix;
pub mod steinhaus_johnson_trotter;

#[derive(Clone, Debug)]
pub struct Permutation {
    pub v: Vec<usize>,
}

impl Permutation {
    pub fn new(len: usize) -> Permutation {
        Permutation {
            v: (0..len).collect(),
        }
    }
}
