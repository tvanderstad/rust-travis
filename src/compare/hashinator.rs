use std::hash::Hasher;

pub trait Hashinator<Operand, OperandHasher: Hasher> {
    fn hash(a: &Operand, s: &mut OperandHasher);
}
