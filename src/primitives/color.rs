use crate::float::ApproxEq;
#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn red(&self) -> f64 {
        self.r
    }

    pub fn green(&self) -> f64 {
        self.g
    }

    pub fn blue(&self) -> f64 {
        self.b
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq_low_precision(other.r)
            && self.g.approx_eq_low_precision(other.g)
            && self.b.approx_eq_low_precision(other.b)
    }
}
impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            b: self.b * rhs.b,
            g: self.g * rhs.g,
        }
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            b: self.b - rhs.b,
            g: self.g - rhs.g,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Color::new(1.0, 2.0, 3.0);
        let b = Color::new(3.0, 4.0, 5.0);
        let result = a + b;
        assert_eq!(result, Color::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn mul() {
        let a = Color::new(1.0, 0.4, 0.3);
        let b = Color::new(0.1, 0.9, 0.2);
        let result = a * b;
        assert_eq!(result, Color::new(0.1, 0.36, 0.06));
    }

    #[test]
    fn sub() {
        let a = Color::new(1.0, 2.0, 3.0);
        let b = Color::new(3.0, 4.0, 5.0);
        let result = a - b;
        assert_eq!(result, Color::new(-2.0, -2.0, -2.0));
    }
}
