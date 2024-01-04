use crate::rtc::{ray::Ray, intersection::Intersections, shapes::sphere::Sphere};

use super::object::Object;
#[derive(Debug, PartialEq)]
pub enum Shape{
   Sphere,
}

impl<'a> Shape{
    pub fn intersect(&self, ray: &'a Ray, object: &'a Object) -> Intersections<'a>{
        match self{
            Shape::Sphere => Sphere::intersect(ray ,object),
        }
    }
}
