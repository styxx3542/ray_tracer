use crate::float::ApproxEq;
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
    max_recursive_depth: u8,
}

impl<'a> World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
            max_recursive_depth: 5,
        }
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;
        self
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn with_lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
        self
    }

    pub fn with_depth(mut self, depth: u8) -> Self {
        self.max_recursive_depth = depth;
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

    pub fn shade_hit(&self, state: &IntersectionState, remaining_recursions: u8) -> Color {
        let object_point = state.object().to_object_space(&state.over_point());
        let shadowed = self.is_shadowed(&state.over_point());
        let reflected = self.reflected_color(state, remaining_recursions);
        let refracted = self.refracted_color(state, remaining_recursions);
        let surface_color: Color = self
            .lights
            .iter()
            .map(|light| {
                state.object().material().lighting(
                    &light,
                    &object_point,
                    &state.over_point(),
                    &state.eyev(),
                    &state.normalv(),
                    shadowed,
                )
            })
            .sum();
        let material = state.object().material();
        if material.reflective() > 0.0 && material.transparency() > 0.0 {
            let reflectance = state.schlick();
            return surface_color + reflected * reflectance + refracted * (1.0 - reflectance);
        }
        surface_color + reflected + refracted
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        let v = self.lights[0].position() - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);
        if let Some(hit) = intersections.hit() {
            hit.t() < distance
        } else {
            false
        }
    }

    pub fn color_at(&self, ray: &mut Ray) -> Color {
        self.color_at_impl(ray, self.max_recursive_depth)
    }

    pub fn color_at_impl(&self, ray: &mut Ray, remaining_recursions: u8) -> Color {
        let xs = self.intersect(ray);
        if let Some(hit) = xs.hit() {
            let state = IntersectionState::prepare_computations(&hit, ray);
            self.shade_hit(&state, remaining_recursions)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn reflected_color(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        if comps.object().material().reflective() == 0.0 || remaining_recursions == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut reflect_ray = Ray::new(comps.over_point(), comps.reflectv());
        let color = self.color_at_impl(&mut reflect_ray, remaining_recursions - 1);
        color * comps.object().material().reflective()
    }

    pub fn refracted_color(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        if comps.object().material().transparency().approx_eq(0.0) || remaining_recursions == 0 {
            return Color::black();
        }
        let n_ratio = comps.n1() / comps.n2();
        let cos_i = comps.eyev().dot_product(&comps.normalv());
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return Color::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv() * (n_ratio * cos_i - cos_t) - comps.eyev() * n_ratio;
        let outside_index = comps.n2();
        let mut refract_ray =
            Ray::new(comps.under_point(), direction).with_indices(vec![outside_index]);
        self.color_at_impl(&mut refract_ray, remaining_recursions - 1)
            * comps.object().material().transparency()
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
            max_recursive_depth: 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{primitives::Vector, rtc::pattern::Pattern};
    use pretty_assertions::assert_eq;
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
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, &shape);
        let state = IntersectionState::prepare_computations(&i, &mut r);
        let c = w.shade_hit(&state, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.lights = vec![PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new(0.0, 0.25, 0.0),
        )];
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let state = IntersectionState::prepare_computations(&i, &mut r);
        let c = w.shade_hit(&state, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&mut r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&mut r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn reflected_color_for_nonreflective_material() {
        let w = World::default();
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let shape = shape
            .clone()
            .set_material(&Material::new().with_ambient(1.0));
        let i = Intersection::new(1.0, &shape);
        let state = IntersectionState::prepare_computations(&i, &mut r);
        let color = w.reflected_color(&state, 1);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let shape = Object::new_plane()
            .set_material(&Material::new().with_reflective(0.5))
            .set_transform(&Matrix::id().translate(0.0, -1.0, 0.0));
        let mut w = World::default();
        w.add_object(shape.clone());
        let mut r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let state = IntersectionState::prepare_computations(&i, &mut r);
        let color = w.shade_hit(&state, 1);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn mutually_reflective_surfaces() {
        let lower = Object::new_plane()
            .set_material(&Material::new().with_reflective(1.0))
            .set_transform(&Matrix::id().translate(0.0, -1.0, 0.0));
        let upper = Object::new_plane()
            .set_material(&Material::new().with_reflective(1.0))
            .set_transform(&Matrix::id().translate(0.0, 1.0, 0.0));
        let mut w = World::default();
        w.add_object(lower.clone());
        w.add_object(upper.clone());
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        w.color_at(&mut r);
        // Make sure program terminates
        assert!(true);
    }

    #[test]
    fn maximum_recursive_depth() {
        let shape = Object::new_plane()
            .set_material(&Material::new().with_reflective(0.5))
            .set_transform(&Matrix::id().translate(0.0, -1.0, 0.0));
        let mut w = World::default();
        w.add_object(shape.clone());
        let mut r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let state = IntersectionState::prepare_computations(&i, &mut r);
        let color = w.reflected_color(&state, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_opaque_surface() {
        let w = World::default();
        let shape = &w.objects[0];
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let state = IntersectionState::prepare_computations(&xs[0], &mut r);
        let color = w.refracted_color(&state, 5);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refraction_at_max_recursive_depth() {
        let w = World::default();
        let shape = &w.objects[0];
        shape.clone().set_material(
            &Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-0.9899, shape),
            Intersection::new(-0.4899, shape),
            Intersection::new(0.4899, shape),
            Intersection::new(0.9899, shape),
        ]);
        let state = IntersectionState::prepare_computations(&xs[2], &mut r);
        let color = w.refracted_color(&state, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_total_internal_refraction() {
        let w = World::default();
        let shape = &w.objects[0];
        shape.clone().set_material(
            &Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        let mut r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape),
        ]);
        let state = IntersectionState::prepare_computations(&xs[1], &mut r);
        let color = w.refracted_color(&state, 5);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color() {
        let w = World::default();
        let a = &w.objects[0];
        let a = a.clone().set_material(
            &Material::new()
                .with_ambient(1.0)
                .with_pattern(Pattern::new_test()),
        );
        let b = &w.objects[1];
        let b = b.clone().set_material(
            &Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0))
            .with_indices(vec![1.0, 1.5]);
        let xs = Intersections::new()
            .with_intersections(vec![
                Intersection::new(-0.9899, &a),
                Intersection::new(-0.4899, &b),
                Intersection::new(0.4899, &b),
                Intersection::new(0.9899, &a),
            ])
            .sort();
        let w = World::default().with_objects(vec![a.clone(), b.clone()]);
        let state = IntersectionState::prepare_computations(&xs[2], &mut r);
        let color = w.refracted_color(&state, 5);
        assert_eq!(color, Color::new(0.0, 0.998888, 0.04725))
    }
    #[test]
    fn shade_hit_transparent_material() {
        let mut w = World::default();
        let floor = Object::new_plane()
            .set_transform(&Matrix::id().translate(0.0, -1.0, 0.0))
            .set_material(
                &Material::new()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            );
        let ball = Object::new_sphere()
            .set_transform(&Matrix::id().translate(0.0, -3.5, -0.5))
            .set_material(
                &Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            );
        w.add_object(floor.clone());
        w.add_object(ball.clone());
        let mut r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new()
            .with_intersections(vec![Intersection::new(2.0_f64.sqrt(), &floor)]);
        let state = IntersectionState::prepare_computations(&xs[0], &mut r);
        let color = w.shade_hit(&state, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_reflective_transparent_material() {
        let mut w = World::default();
        let floor = Object::new_plane()
            .set_transform(&Matrix::id().translate(0.0, -1.0, 0.0))
            .set_material(
                &Material::new()
                    .with_reflective(0.5)
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            );
        let ball = Object::new_sphere()
            .set_transform(&Matrix::id().translate(0.0, -3.5, -0.5))
            .set_material(
                &Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            );
        w.add_object(floor.clone());
        w.add_object(ball.clone());
        let mut r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new()
            .with_intersections(vec![Intersection::new(2.0_f64.sqrt(), &floor)]);
        let state = IntersectionState::prepare_computations(&xs[0], &mut r);
        let color = w.shade_hit(&state, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
