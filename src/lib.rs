pub fn moore_neighborhood((y, x): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    [
        (y.wrapping_sub(1), x.wrapping_sub(1)),
        (y.wrapping_sub(1), x),
        (y.wrapping_sub(1), x + 1),
        (y, x.wrapping_sub(1)),
        (y, x + 1),
        (y + 1, x.wrapping_sub(1)),
        (y + 1, x),
        (y + 1, x + 1),
    ]
        .into_iter()
        .filter(|&(ny, nx)| ny < height && nx < width)
        .collect()
}