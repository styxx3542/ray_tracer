use crate::primitives::{Color, Matrix, Point, Tuple};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Matrix,
    transform_inverse: Matrix, // caching purposes
}

impl Pattern {
    pub fn new_test() -> Pattern {
        Pattern {
            pattern_type: PatternType::Test(TestPattern {}),
            ..Default::default()
        }
    }

    pub fn new_stripe(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Stripe(StripePattern { a, b }),
            ..Default::default()
        }
    }

    pub fn stripe_at(&self, object_point: &Point) -> Color {
        let pattern_point = self.to_pattern_space(object_point);
        match self.pattern_type {
            PatternType::Stripe(p) => p.stripe_at(&pattern_point),
            PatternType::Test(p) => p.stripe_at(&pattern_point),
            _ => unimplemented!(),
        }
    }

    pub fn set_transform(mut self, transform: Matrix) -> Self {
        self.transform = transform;
        self.transform_inverse = transform.inverse().unwrap();
        self
    }

    pub fn to_pattern_space(&self, object_point: &Point) -> Point {
        self.transform_inverse * *object_point
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            pattern_type: PatternType::Test(TestPattern {}),
            transform: Matrix::id(),
            transform_inverse: Matrix::id(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PatternType {
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring,
    Checkers,
    Test(TestPattern),
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    fn stripe_at(&self, point: &Point) -> Color {
        if (point.x().floor() as i64 % 2) == 0 {
            return self.a;
        }
        self.b
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct GradientPattern {
    a: Color,
    b: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TestPattern {}
impl TestPattern {
    fn stripe_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::rtc::object::Object;

    use super::*;

    #[test]
    fn pattern_stripe_constant_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn pattern_stripe_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn pattern_stripe_alternating_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripe_with_object_transformation() {
        let sphere = Object::new_sphere().set_transform(&Matrix::id().scale(2.0, 2.0, 2.0));
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let mut pattern = Pattern::new_stripe(white, black);
        pattern = pattern.set_transform(Matrix::id().scale(2.0, 2.0, 2.0));
        let point = Point::new(1.5, 0.0, 0.0);
        let object_point = sphere.to_object_space(&point);
        assert_eq!(pattern.stripe_at(&object_point), white);
    }

    #[test]
    fn stripe_with_pattern_transformation() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern =
            Pattern::new_stripe(white, black).set_transform(Matrix::id().scale(2.0, 2.0, 2.0));
        assert_eq!(pattern.stripe_at(&Point::new(1.5, 0.0, 0.0)), white);
    }

    #[test]
    fn stripe_with_both_object_and_pattern_transformation() {
        let sphere = Object::new_sphere().set_transform(&Matrix::id().scale(2.0, 2.0, 2.0));
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let mut pattern = Pattern::new_stripe(white, black);
        pattern = pattern.set_transform(Matrix::id().translate(0.5, 0.0, 0.0));
        let point = Point::new(2.5, 0.0, 0.0);
        let object_point = sphere.to_object_space(&point);
        assert_eq!(pattern.stripe_at(&object_point), white);
    }
}
