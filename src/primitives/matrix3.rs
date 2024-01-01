use crate::{float::ApproxEq, primitives::matrix2::Matrix2};
use std::ops::{Index, IndexMut};
const MATRIX_SIZE: usize = 3;

pub struct Matrix3 {
    grid: [f64; MATRIX_SIZE * MATRIX_SIZE],
}
impl Index<(usize, usize)> for Matrix3 {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 {
            grid: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut result = Matrix2::new();
        let mut result_row = 0;
        let mut result_col = 0;
        for i in 0..MATRIX_SIZE {
            if i == row {
                continue;
            }
            for j in 0..MATRIX_SIZE {
                if j == col {
                    continue;
                }
                result[(result_row, result_col)] = self[(i, j)];
                result_col += 1;
            }
            result_row += 1;
            result_col = 0;
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn colfactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..MATRIX_SIZE {
            result += self[(0, i)] * self.colfactor(0, i);
        }
        result
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}
impl std::ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Self::Output {
        let mut result = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                result[(i, j)] = self[(i, 0)] * rhs[(0, j)]
                    + self[(i, 1)] * rhs[(1, j)]
                    + self[(i, 2)] * rhs[(2, j)];
            }
        }
        result
    }
}

impl PartialEq for Matrix3 {
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
        let mut a = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let mut b = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                b[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let c = a * b;
        assert_eq!(c[(0, 0)], 15.0);
        assert_eq!(c[(0, 1)], 18.0);
    }
    #[test]
    fn test_submatrix() {
        let mut a = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let b = a.submatrix(0, 0);
        assert_eq!(b[(0, 0)], 4.0);
        assert_eq!(b[(0, 1)], 5.0);
        assert_eq!(b[(1, 0)], 7.0);
        assert_eq!(b[(1, 1)], 8.0);
    }

    #[test]
    fn test_minor() {
        let mut a = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let b = a.minor(1, 0);
        assert_eq!(b, a.submatrix(1, 0).determinant());
    }

    #[test]
    fn test_determinant() {
        let mut a = Matrix3::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        assert_eq!(a.determinant(), 0.0);
    }
}
