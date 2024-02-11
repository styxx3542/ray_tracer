use crate::primitives::{Color, Point, Vector};
use crate::rtc::{light::PointLight, pattern::Pattern};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pattern: Option<Pattern>,
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    does_cast_shadow: bool,   
}

impl Material {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn pattern(&self) -> Option<Pattern> {
        Some(self.pattern)?
    }

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn does_cast_shadow(&self) -> bool {
        self.does_cast_shadow
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
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
    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Self{
        self.reflective = reflective;
        self
    }

    pub fn with_shadow(mut self, shadow: bool) -> Self{
        self.does_cast_shadow = shadow;
        self
    }


    pub fn lighting(
        &self,
        light: &PointLight,
        object_point: &Point,
        world_point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match self.pattern {
            Some(pattern) => pattern.pattern_at(object_point),
            None => self.color,
        };
        let effective_color = color * light.intensity();
        let lightv = (light.position() - *world_point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot_product(normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 || (in_shadow && self.does_cast_shadow()) {
            (Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0))
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot_product(eyev);
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
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            does_cast_shadow: true,
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
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_wigh_surface_in_shadow() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let in_shadow = true;
        let result = m.lighting(&light, &position, &position, &eyev, &normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut m = Material::new();
        m.pattern = Some(Pattern::new_stripe(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let mut object_point = Point::new(0.9, 0.0, 0.0);
        let c1 = m.lighting(&light, &object_point, &object_point, &eyev, &normalv, false);
        object_point = Point::new(1.1, 0.0, 0.0);
        let c2 = m.lighting(&light, &object_point, &object_point, &eyev, &normalv, false);
        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }
}
