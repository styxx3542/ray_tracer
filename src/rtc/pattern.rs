use crate::{primitives::{Color, Matrix, Point, Tuple}, float::ApproxEq};

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

    pub fn new_gradient(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Gradient(GradientPattern { a, b }),
            ..Default::default()
        }
    }

    pub fn new_ring(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Ring(RingPattern { a, b }),
            ..Default::default()
        }
    }

    pub fn new_checkers(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern_type: PatternType::Checkers(CheckersPattern { a, b }),
            ..Default::default()
        }
    }

    pub fn pattern_at(&self, object_point: &Point) -> Color {
        let pattern_point = self.to_pattern_space(object_point);
        match self.pattern_type {
            PatternType::Stripe(p) => p.pattern_at(&pattern_point),
            PatternType::Test(p) => p.pattern_at(&pattern_point),
            PatternType::Gradient(p) => p.pattern_at(&pattern_point),
            PatternType::Ring(p) => p.pattern_at(&pattern_point),
            PatternType::Checkers(p) => p.pattern_at(&pattern_point),
            PatternType::RadialGradient(p) => p.pattern_at(&pattern_point),
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

trait PatternAt {
    fn pattern_at(&self, point: &Point) -> Color;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PatternType {
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkers(CheckersPattern),
    Test(TestPattern),
    RadialGradient(RadialGradientPattern),
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct StripePattern {
    a: Color,
    b: Color,
}

impl PatternAt for StripePattern {
    fn pattern_at(&self, point: &Point) -> Color {
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

impl PatternAt for GradientPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x() - point.x().floor();
        self.a + distance * fraction
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct RingPattern{
    a: Color,
    b: Color,
}

impl PatternAt for RingPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        if (point.x().powi(2) + point.z().powi(2)).sqrt().floor() as i64 % 2 == 0 {
            return self.a;
        }
        self.b
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct CheckersPattern {
    a: Color,
    b: Color,
}

impl PatternAt for CheckersPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let sum = point.x().floor() + point.y().floor() + point.z().floor();
        if (sum % 2.0).approx_eq(0.0) {
            return self.a;
        }
        self.b
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct RadialGradientPattern{
    a: Color,
    b: Color,
}


impl PatternAt for RadialGradientPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x().powi(2) + point.z().powi(2);
        let fraction = fraction.sqrt() - point.y().floor();
        self.a + distance * fraction
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct TestPattern {}
impl PatternAt for TestPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::rtc::{material::Material, object::Object};

    use super::*;

    #[test]
    fn pattern_stripe_constant_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn pattern_stripe_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn pattern_stripe_alternating_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern = Pattern::new_stripe(white, black);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(&Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(&Point::new(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(&Point::new(-1.1, 0.0, 0.0)), white);
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
        assert_eq!(pattern.pattern_at(&object_point), white);
    }

    #[test]
    fn stripe_with_pattern_transformation() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let pattern =
            Pattern::new_stripe(white, black).set_transform(Matrix::id().scale(2.0, 2.0, 2.0));
        assert_eq!(pattern.pattern_at(&Point::new(1.5, 0.0, 0.0)), white);
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
        assert_eq!(pattern.pattern_at(&object_point), white);
    }

    #[test]
    fn test_pattern() {
        let pattern = Pattern::new_test();
        assert_eq!(pattern.transform, Matrix::id());
    }

    #[test]
    fn test_pattern_set_transform() {
        let pattern = Pattern::new_test().set_transform(Matrix::id().translate(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform, Matrix::id().translate(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let pattern = Pattern::new_test().set_transform(Matrix::id().translate(0.5, 1.0, 1.5));
        let sphere = Object::new_sphere()
            .set_transform(&Matrix::id().scale(2.0, 2.0, 2.0))
            .set_material(&Material::new().with_pattern(pattern));
        let point = Point::new(2.5, 3.0, 3.5);
        let object_point = sphere.to_object_space(&point);
        assert_eq!(
            sphere
                .material()
                .pattern()
                .unwrap()
                .pattern_at(&object_point),
            Color::new(0.75, 0.5, 0.25)
        );
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::new_gradient(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::new(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    } 

    #[test]
    fn ring_pattern() {
        let pattern = Pattern::new_ring(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.708, 0.0, 0.708)), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::new_checkers(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.99, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(1.01, 0.0, 0.0)), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::new_checkers(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.99, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 1.01, 0.0)), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::new_checkers(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.99)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.01)), Color::new(0.0, 0.0, 0.0));
    }
}
