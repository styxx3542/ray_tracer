use crate::primitives::{Color, Matrix, Point, Tuple};
use crate::rtc::{
    intersection::{Intersection, IntersectionState, Intersections},
    light::PointLight,
    material::Material,
    object::Object,
    ray::Ray,
};

pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl<'a> World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self{
        self.objects = objects;
        self
    }

    pub fn with_lights(mut self, lights: Vec<PointLight>) -> Self{
        self.lights = lights;
        self
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn intersect(&'a self, ray: &Ray) -> Intersections<'a> {
        let mut intersections: Vec<Intersection<'a>> = vec![];
        for object in &self.objects {
            intersections.append(&mut object.intersect(ray).into_iter().collect())
        }
        Intersections::new()
            .with_intersections(intersections)
            .sort()
    }

    pub fn shade_hit(&self, state: &IntersectionState) -> Color {
        self.lights.iter().map(|light|{
            state.object().material().lighting(
            &light,
            &state.point(),
            &state.eyev(),
            &state.normalv(),
        )
        }).sum()
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        if let Some(hit) = xs.hit() {
            let state = IntersectionState::prepare_computations(&hit, ray);
            self.shade_hit(&state)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
        let mut s1 = Object::new_sphere();
        s1 = s1.set_material(
            &Material::new()
                .with_color(Color::new(0.8, 1.0, 0.6))
                .with_diffuse(0.7)
                .with_specular(0.2),
        );
        let mut s2 = Object::new_sphere();
        s2 = s2.set_transform(&Matrix::id().scale(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::primitives::Vector;
    #[test]
    fn test_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn test_default_world() {
        let w = World::default();
        assert_eq!(
            w.lights[0],
            PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0))
        );
        assert_eq!(w.objects[0].material().color(), Color::new(0.8, 1.0, 0.6));
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, &shape);
        let state = IntersectionState::prepare_computations(&i, &r);
        let c = w.shade_hit(&state);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.lights = vec![PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new(0.0, 0.25, 0.0),
        )];
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let state = IntersectionState::prepare_computations(&i, &r);
        let c = w.shade_hit(&state);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
}
