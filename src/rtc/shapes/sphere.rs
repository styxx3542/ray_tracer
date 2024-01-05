use crate::rtc::intersection::Intersections;
use crate::rtc::object::Object;
use crate::rtc::ray::Ray;
use crate::primitives::Point;
use crate::primitives::Tuple;
#[derive(Debug, Copy, Clone)]
pub struct Sphere{} 

impl<'a> Sphere{
    pub fn intersect(ray: &Ray, object: &'a Object) -> Intersections<'a>{
        let mut intersections = Intersections::new();
        let sphere_to_ray = Point::zero() - ray.origin;
        let tc = sphere_to_ray.dot_product(ray.direction.normalize());
        let l = sphere_to_ray.dot_product(sphere_to_ray);
        let d2 = l - tc * tc;
        if d2 > 1.0 {
            return Intersections::new(); 
        }
        let del_t = (1.0 - d2).sqrt() / ray.direction.magnitude();
        let tc = tc / ray.direction.magnitude();
        let t1 = tc - del_t;
        let t2 = tc + del_t;
        intersections.push(object, t1);
        intersections.push(object, t2);
        intersections
    }
}

