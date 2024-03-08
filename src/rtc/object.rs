use crate::{
    primitives::{Matrix, Point, Vector},
    rtc::shape::Shape,
};

use super::{intersection::Intersections, material::Material, ray::Ray};
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    transform: Matrix,
    transform_inverse: Matrix,
    transform_inverse_transpose: Matrix,
    material: Material,
}

impl<'a> Object {
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere,
            ..Default::default()
        }
    }

    pub fn new_glass_sphere() -> Self {
        Object {
            shape: Shape::Sphere,
            ..Default::default()
        }
        .set_material(
            &Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        )
    }

    pub fn to_object_space(&self, world_point: &Point) -> Point {
        self.transform_inverse * *world_point
    }

    pub fn new_cylinder(minimum: f64, maximum: f64) -> Self {
        Object {
            shape: Shape::Cylinder(minimum, maximum, false),
            ..Default::default()
        }
    }
    pub fn new_closed_cylinder(minimum: f64, maximum: f64) -> Self {
        Object {
            shape: Shape::Cylinder(minimum, maximum, true),
            ..Default::default()
        }
    }
    pub fn new_closed_cone(minimum: f64, maximum: f64) -> Self {
        Object {
            shape: Shape::Cylinder(minimum, maximum, true),
            ..Default::default()
        }
    }

    pub fn new_cone(minimum: f64, maximum: f64) -> Self {
        Object {
            shape: Shape::Cone(minimum, maximum, false),
            ..Default::default()
        }
    }

    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane,
            ..Default::default()
        }
    }

    pub fn new_cube() -> Self {
        Object {
            shape: Shape::Cube,
            ..Default::default()
        }
    }
    pub fn material(&self) -> Material {
        self.material
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }
    pub fn intersect(&self, ray: &'a Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.transform_inverse);
        self.shape.intersect(&transformed_ray, self)
    }

    pub fn set_transform(mut self, transform: &Matrix) -> Self {
        self.transform = *transform;
        self.transform_inverse = (*transform).inverse().unwrap();
        self.transform_inverse_transpose = self.transform_inverse.transpose();
        self
    }
    pub fn set_material(mut self, material: &Material) -> Self {
        self.material = *material;
        self
    }
    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.to_object_space(world_point);
        let object_normal = self.shape.normal_at(&object_point);
        let world_normal = self.transform_inverse_transpose * object_normal; //convert normal back to world space
        world_normal.normalize()
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }
    pub fn transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }
}

impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Shape::Sphere,
            transform: Matrix::id(),
            transform_inverse: Matrix::id(),
            transform_inverse_transpose: Matrix::id(),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Tuple;
    #[test]
    fn intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections[0].object(), &sphere);
        assert_eq!(intersections[1].object(), &sphere);
    }

    #[test]
    fn tangent_intersection() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections[0].t(), 5.0);
        assert_eq!(intersections[1].t(), 5.0);
    }

    #[test]
    fn ray_miss() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 0);
    }

    #[test]
    fn ray_inside_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections[0].t(), -1.0);
        assert_eq!(intersections[1].t(), 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections[0].t(), -6.0);
        assert_eq!(intersections[1].t(), -4.0);
    }

    #[test]
    fn default_sphere() {
        let sphere = Object::new_sphere();
        assert_eq!(sphere.transform, Matrix::id());
    }

    #[test]
    fn change_sphere_transform() {
        let mut sphere = Object::new_sphere();
        let transform = Matrix::id().translate(2.0, 3.0, 4.0);
        sphere = sphere.set_transform(&transform);
        assert_eq!(sphere.transform, transform);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Object::new_sphere();
        let transform = Matrix::id().scale(2.0, 2.0, 2.0);
        sphere = sphere.set_transform(&transform);
        assert_eq!(sphere.transform, transform);
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections[0].t(), 3.0);
        assert_eq!(intersections[1].t(), 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Object::new_sphere();
        let transform = Matrix::id().translate(5.0, 0.0, 0.0);
        sphere = sphere.set_transform(&transform);
        assert_eq!(sphere.transform, transform);
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 0);
    }
}
