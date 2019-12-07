pub trait EqualityComparator<Operand> {
    fn are_equal(a: Operand, b: Operand) -> bool;
}
