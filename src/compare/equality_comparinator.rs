pub trait EqualityComparinator<Operand> {
    fn are_equal(a: Operand, b: Operand) -> bool;
}
