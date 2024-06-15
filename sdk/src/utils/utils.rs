pub const fn const_max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}
