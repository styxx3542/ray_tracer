use crate::{
    float::ApproxEq,
    primitives::{tuple::Tuple, vector::Vector},
};
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn w(&self) -> f64 {
        1.0
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
    fn zero() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq_low_precision(other.x)
            && self.y.approx_eq_low_precision(other.y)
            && self.z.approx_eq_low_precision(other.z)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let result = Point::new(1.0, 1.0, 6.0);
        assert_eq!(p + v, result);
    }
    #[test]
    fn subtract_point() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let result = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, result);
    }
    #[test]
    fn subtract_vector() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let result = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(p - v, result);
    }

    #[test]
    fn scalar_multiplication() {
        let p = Point::new(1.0, -2.0, 3.0);
        let result = Point::new(3.5, -7.0, 10.5);
        assert_eq!(p * 3.5, result);
        assert_eq!(p * 1.0, p);
        assert_eq!(p * 0.5, Point::new(0.5, -1.0, 1.5));
        assert_eq!(p * 0.0, Point::zero());
    }
}
