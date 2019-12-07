pub trait Hasher<Operand> {
    fn hash(a: Operand) -> u64;
}
