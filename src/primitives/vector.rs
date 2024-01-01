use crate::{float::ApproxEq, primitives::tuple::Tuple};
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot_product(&self, other: Vector) -> f64 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    pub fn cross_product(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z() - self.z * other.y(),
            y: self.z * other.x() - self.x * other.z(),
            z: self.x * other.y() - self.y * other.x(),
        }
    }
}
impl Tuple for Vector {
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
        0.0
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }
    fn zero() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq_low_precision(other.x)
            && self.y.approx_eq_low_precision(other.y)
            && self.z.approx_eq_low_precision(other.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x(),
            y: -self.y(),
            z: -self.z(),
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x(), 4.3);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);
    }
    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }
    #[test]
    fn test_vector_subtraction() {
        let p1 = Vector::new(3.0, 2.0, 1.0);
        let p2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }
    #[test]
    fn test_vector_negation() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }
    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 1.5));
        assert_eq!(v * 0.0, Vector::zero());
        assert_eq!(v * 1.0, v);
    }
    #[test]
    fn test_vector_magnitude() {
        assert_eq!(Vector::new(1.0, 2.0, 4.0).magnitude(), 21.0f64.sqrt());
        assert_eq!(Vector::new(-1.0, -2.0, -4.0).magnitude(), 21.0f64.sqrt());
        assert_eq!(Vector::new(0.0, 0.0, 0.0).magnitude(), 0.0);
    }
    #[test]
    fn test_vector_normalize() {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).normalize(),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0).normalize(),
            Vector::new(0.26726, 0.53452, 0.80178)
        );
        assert_eq!(Vector::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }
    #[test]
    fn test_dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot_product(b), 20.0);
    }
    #[test]
    fn test_cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(a.cross_product(b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross_product(a), Vector::new(1.0, -2.0, 1.0));
    }
}
