use crate::{
    primitives::{Point, Vector},
    rtc::{intersection::Intersections, ray::Ray, shapes::sphere::Sphere},
};

use super::{object::Object, shapes::plane::Plane};
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
}

impl<'a> Shape {
    pub fn intersect(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere => Sphere::intersect(ray, object),
            Shape::Plane => Plane::intersects(ray, object),
        }
    }
    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Sphere => Sphere::normal_at(object_point),
            Shape::Plane => Plane::normal_at(object_point),
        }
    }
}
