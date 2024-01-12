use crate::{rtc::{ray::Ray, intersection::Intersections, shapes::sphere::Sphere}, primitives::{Vector, Point}};

use super::object::Object;
#[derive(Clone, Debug, PartialEq)]
pub enum Shape{
   Sphere,
}

impl<'a> Shape{
    pub fn intersect(&self, ray: &Ray, object: &'a Object) -> Intersections<'a>{
        match self{
            Shape::Sphere => Sphere::intersect(ray ,object),
        }
    }
    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self{
            Shape::Sphere => Sphere::normal_at(object_point)
        }
    } 
}
