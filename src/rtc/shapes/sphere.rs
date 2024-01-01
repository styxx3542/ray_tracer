use crate::rtc::ray::Ray;
use crate::primitives::Point;
use crate::primitives::Tuple;
#[derive(Debug, Copy, Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn intersects(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = Point::new(0.0, 0.0, 0.0) - ray.origin;
        let tc = sphere_to_ray.dot_product(ray.direction);
        let l = sphere_to_ray.dot_product(sphere_to_ray);
        let d2 = l - tc * tc;
        if d2 > 1.0 {
            return vec![];
        }
        let del_t = (1.0 - d2).sqrt();
        let t1 = tc - del_t;
        let t2 = tc + del_t;
         vec![t1,t2]
    }
}
#[cfg(test)]
mod tests{
    use crate::primitives::Vector;

    use super::*;
    #[test]
    fn intersection(){
        let ray = Ray::new(Point::new(0.0,0.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere{};
        let intersections = sphere.intersects(&ray);
        assert_eq!(intersections.len(), 2);  
        assert_eq!(intersections[0], 4.0);
        assert_eq!(intersections[1], 6.0);
    }

    #[test]
    fn tangent_intersection(){
        let ray = Ray::new(Point::new(0.0,1.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere{};
        let intersections = sphere.intersects(&ray);
        assert_eq!(intersections.len(), 2);  
        assert_eq!(intersections[0], 5.0);
        assert_eq!(intersections[1], 5.0);
    }

    #[test]
    fn ray_miss(){
        let ray = Ray::new(Point::new(0.0,2.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere{};
        let intersections = sphere.intersects(&ray);
        assert_eq!(intersections.len(), 0);  
    }

    #[test]
    fn ray_inside_sphere(){
        let ray = Ray::new(Point::new(0.0,0.0,0.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere{};
        let intersections = sphere.intersects(&ray);
        assert_eq!(intersections.len(), 2);  
        assert_eq!(intersections[0], -1.0);
        assert_eq!(intersections[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray(){

        let ray = Ray::new(Point::new(0.0,0.0,5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere{};
        let intersections = sphere.intersects(&ray);
        assert_eq!(intersections.len(), 2);  
        assert_eq!(intersections[0], -6.0);
        assert_eq!(intersections[1], -4.0);
    }
}
