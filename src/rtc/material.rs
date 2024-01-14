use crate::primitives::{Color, Point, Vector};

use super::light::PointLight;
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color(&self) -> Color{
        self.color
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }
    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
    ) -> Color {
        let effective_color = self.color * light.intensity();
        let lightv = (light.position() - *point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot_product(*normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0))
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot_product(*eyev);
            let specular = if reflect_dot_eye <= 0.0 {
                Color::new(0.0, 0.0, 0.0)
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity() * self.specular * factor
            };
            (diffuse, specular)
        };
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Tuple;
    #[test]
    fn test_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
