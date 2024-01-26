use crate::{
    primitives::{Color, Point, Tuple},
};

pub struct Pattern {
    a: Color,
    b: Color,
}

impl Pattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern { a, b }
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if (point.x().floor() as i64 % 2) == 0 {
            return self.a;
        }
        self.b
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_create() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new(white, black);
        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    fn pattern_stripe_constant_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn pattern_stripe_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn pattern_stripe_alternating_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.1, 0.0, 0.0)), white);
    }
}
