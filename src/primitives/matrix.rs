use crate::{
    float::ApproxEq,
    primitives::{matrix3::Matrix3, tuple::Tuple},
};
use std::ops::{Index, IndexMut};
const MATRIX_SIZE: usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    grid: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0 * MATRIX_SIZE + index.1]
    }
}
impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            grid: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn from_array(grid: [f64; MATRIX_SIZE * MATRIX_SIZE]) -> Matrix {
        Matrix { grid }
    }

    pub fn id() -> Matrix {
        let mut grid = [0.0; MATRIX_SIZE * MATRIX_SIZE];
        grid[5] = 1.0;
        grid[0] = 1.0;
        grid[10] = 1.0;
        grid[15] = 1.0;
        Matrix { grid }
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                result[(i, j)] = self[(j, i)];
            }
        }
        result
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut result = Matrix3::new();
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

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.submatrix(row, col).determinant()
        } else {
            -self.submatrix(row, col).determinant()
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..MATRIX_SIZE {
            result += self[(0, i)] * self.cofactor(0, i);
        }
        result
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn inverse(&self) -> Option<Matrix> {
        let mut result = Matrix::new();
        if self.invertible() == false {
            return None;
        }

        let det = self.determinant();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                let c = self.cofactor(i, j);
                result[(j, i)] = c / det;
            }
        }
        Some(result)
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(0, 3)] = x;
        result[(1, 3)] = y;
        result[(2, 3)] = z;
        result * *self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(0, 0)] = x;
        result[(1, 1)] = y;
        result[(2, 2)] = z;
        result * *self
    }

    pub fn rotate_x(&self, r: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(1, 1)] = r.cos();
        result[(1, 2)] = -r.sin();
        result[(2, 1)] = r.sin();
        result[(2, 2)] = r.cos();
        result * *self
    }

    pub fn rotate_y(&self, r: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(0, 0)] = r.cos();
        result[(0, 2)] = r.sin();
        result[(2, 0)] = -r.sin();
        result[(2, 2)] = r.cos();
        result * *self
    }

    pub fn rotate_z(&self, r: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(0, 0)] = r.cos();
        result[(0, 1)] = -r.sin();
        result[(1, 0)] = r.sin();
        result[(1, 1)] = r.cos();
        result
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut result = Matrix::id();
        result[(0, 1)] = xy;
        result[(0, 2)] = xz;
        result[(1, 0)] = yx;
        result[(1, 2)] = yz;
        result[(2, 0)] = zx;
        result[(2, 1)] = zy;
        result * *self
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                result[(i, j)] = self[(i, 0)] * rhs[(0, j)]
                    + self[(i, 1)] * rhs[(1, j)]
                    + self[(i, 2)] * rhs[(2, j)]
                    + self[(i, 3)] * rhs[(3, j)];
            }
        }
        result
    }
}

impl<T> std::ops::Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
            self[(0, 0)] * rhs.x()
                + self[(0, 1)] * rhs.y()
                + self[(0, 2)] * rhs.z()
                + self[(0, 3)] * rhs.w(),
            self[(1, 0)] * rhs.x()
                + self[(1, 1)] * rhs.y()
                + self[(1, 2)] * rhs.z()
                + self[(1, 3)] * rhs.w(),
            self[(2, 0)] * rhs.x()
                + self[(2, 1)] * rhs.y()
                + self[(2, 2)] * rhs.z()
                + self[(2, 3)] * rhs.w(),
        )
    }
}

impl PartialEq for Matrix {
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
    use crate::primitives::{point::Point, vector::Vector};
    #[test]
    fn test_matrix_multiplication() {
        let mut a = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let mut b = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                b[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let c = a * b;
        assert_eq!(c[(0, 0)], 56.0);
        assert_eq!(c[(0, 1)], 62.0);
    }
    #[test]
    fn test_identity_matrix() {
        let mut a = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let b = Matrix::id();
        assert_eq!(a * b, a);
    }

    #[test]
    fn test_inverse() {
        let mut a = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                a[(i, j)] = (i * MATRIX_SIZE + j) as f64;
            }
        }
        let b = a.inverse();
        assert_eq!(b, None);
        let a = Matrix::from_array([
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);
        let b = a.inverse().unwrap();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);
    }

    #[test]
    fn test_matrix_product_invertibility() {
        let a = Matrix::from_array([
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        ]);

        let b = Matrix::from_array([
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[test]
    fn test_translate() {
        let transform = Matrix::id().translate(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_scaling() {
        let transform = Matrix::id().scale(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Vector::new(-8.0, 18.0, 32.0));
        let transform = Matrix::id().scale(2.0, 3.0, 4.0);
        assert_eq!(
            transform.inverse().unwrap() * v,
            Vector::new(-2.0, 2.0, 2.0)
        );
        let transform = Matrix::id().scale(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotation() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::id().rotate_x(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::id().rotate_x(std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::id().rotate_y(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::id().rotate_y(std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::id().rotate_z(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::id().rotate_z(std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing() {
        let transform = Matrix::id().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
        let transform = Matrix::id().shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
        let transform = Matrix::id().shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0));
        let transform = Matrix::id().shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
        let transform = Matrix::id().shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0));
        let transform = Matrix::id().shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_chain_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::id().rotate_x(std::f64::consts::PI / 2.0);
        let b = Matrix::id().scale(5.0, 5.0, 5.0);
        let c = Matrix::id().translate(10.0, 5.0, 7.0);
        let p2 = a * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        let p3 = b * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        let p4 = c * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
        let t = c * b * a;
        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0));
        let chained = Matrix::id()
            .rotate_x(std::f64::consts::PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(chained* p, t * p);  
    }
}
