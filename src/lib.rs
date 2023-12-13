use std::slice::Iter;

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

pub fn transpose<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut row_iters: Vec<Iter<T>> = matrix
        .into_iter()
        .map(|r| r.into_iter())
        .collect();

    (0..matrix.first().unwrap().len())
        .map(|_| row_iters
            .iter_mut()
            .map(|it| it.next().unwrap().clone())
            .collect::<Vec<T>>()
        )
        .collect()
}

pub fn count_different_elements<T: Eq>(a: &[T], b: &[T]) -> usize {
    a
        .iter()
        .zip(b)
        .filter(|&(element_a, element_b) | element_a != element_b)
        .count()
}
