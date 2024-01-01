use crate::float::ApproxEq;
use std::ops::{Index, IndexMut};
const MATRIX_SIZE: usize = 2;
pub struct Matrix2 {
    grid: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Index<(usize, usize)> for Matrix2 {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix2 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}

impl Matrix2 {
    pub fn new() -> Matrix2 {
        Matrix2 {
            grid: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn determinant(&self) -> f64 {
        self.grid[0] * self.grid[3] - self.grid[1] * self.grid[2]
    }
}
impl std::ops::Mul<Matrix2> for Matrix2 {
    type Output = Matrix2;
    fn mul(self, rhs: Matrix2) -> Self::Output {
        let mut result = Matrix2::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                result[(i, j)] = self[(i, 0)] * rhs[(0, j)] + self[(i, 1)] * rhs[(1, j)];
            }
        }
        result
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.grid
            .iter()
            .zip(other.grid.iter())
            .all(|(a, b)| a.approx_eq_low_precision(*b))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_multiplication() {
        let mut a = Matrix2::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let mut b = Matrix2::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                b[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let c = a * b;
        assert_eq!(c[(0, 0)], 2.0);
        assert_eq!(c[(0, 1)], 3.0);
    }
    #[test]
    fn test_determinant() {
        let mut a = Matrix2::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        assert_eq!(a.determinant(), -2.0);
    }
}
