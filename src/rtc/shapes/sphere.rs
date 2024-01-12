use crate::primitives::Vector;
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
    pub fn normal_at(point: &Point) -> Vector{
        *point - Point::zero()
    } 
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::primitives::Matrix;
    #[test]
    fn normal_at_point_on_x_axis(){
        let n = Sphere::normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_point_on_y_axis(){
        let n = Sphere::normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_point_on_z_axis(){
        let n = Sphere::normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_at_non_axial_point(){
        let n = Sphere::normal_at(&Point::new(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));
        assert_eq!(n, Vector::new(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));
    }

    #[test]
    fn normal_on_translated_sphere(){
        let mut s = Object::new_sphere();
        let translate = Matrix::id().translate(0.0, 1.0, 0.0);
        s = s.set_transform(&translate);
        let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere(){
        let mut s = Object::new_sphere();
        let transform = Matrix::id().rotate_z(std::f64::consts::PI/5.0).scale(1.0, 0.5, 1.0);
        s = s.set_transform(&transform);
        let n = s.normal_at(&Point::new(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}

