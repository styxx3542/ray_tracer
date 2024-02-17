use crate::{
    primitives::{Point, Vector},
    rtc::{
        intersection::Intersections,
        object::Object,
        ray::Ray,
        shapes::{plane::Plane, sphere::Sphere, cube::Cube},
    },
};

use super::shapes::cylinder::Cylinder;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder(f64, f64),
}

impl<'a> Shape {
    pub fn intersect(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere => Sphere::intersect(ray, object),
            Shape::Plane => Plane::intersects(ray, object),
            Shape::Cube => Cube::intersects(ray, object),
            Shape::Cylinder(minimum, maximum) => Cylinder::new(*minimum, *maximum).intersects(ray, object),
        }
    }
    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Sphere => Sphere::normal_at(object_point),
            Shape::Plane => Plane::normal_at(object_point),
            Shape::Cube => Cube::normal_at(object_point),
            Shape::Cylinder(minimum, maximum) => Cylinder::new(*minimum, *maximum).normal_at(object_point),
        }
    }
}
