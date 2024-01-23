use crate::{
    float::epsilon,
    primitives::{Point, Tuple, Vector},
    rtc::intersection::Intersections,
    rtc::object::Object,
    rtc::ray::Ray,
};
pub struct Plane {}

impl<'a> Plane {
    pub fn normal_at(_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn intersects(ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let mut intersections = Intersections::new();
        if ray.direction().y().abs() < epsilon::EPSILON {
            return intersections;
        }
        let t = -ray.origin().y() / ray.direction().y();
        intersections.push(object, t);
        intersections
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_is_constant_on_a_plane() {
        let n1 = Plane::normal_at(&Point::new(0.0, 0.0, 0.0));
        let n2 = Plane::normal_at(&Point::new(10.0, 0.0, -10.0));
        let n3 = Plane::normal_at(&Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let plane = Object::new_plane();
        let xs = Plane::intersects(&ray, &plane);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let plane = Object::new_plane();
        let xs = Plane::intersects(&ray, &plane);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let plane = Object::new_plane();
        let xs = Plane::intersects(&ray, &plane);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].object(), &plane);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let plane = Object::new_plane();
        let xs = Plane::intersects(&ray, &plane);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].object(), &plane);
    }


}
