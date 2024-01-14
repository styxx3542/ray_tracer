use crate::primitives::{Color, Point};

#[derive(PartialEq, Debug)]
pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        PointLight {
            intensity,
            position,
        }
    }
    pub fn position(&self) -> Point{
        self.position
    }
    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Tuple;
    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(intensity, position);
        assert_eq!(light.intensity(), intensity);
        assert_eq!(light.position(), position);
    }
}
