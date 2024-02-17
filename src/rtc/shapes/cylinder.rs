use crate::float::ApproxEq;
use crate::primitives::{Point, Tuple, Vector};
use crate::rtc::{intersection::Intersections, object::Object, ray::Ray};
#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    minimum: f64,
    maximum: f64,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }
}

impl<'a> Cylinder {
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Cylinder { minimum, maximum }
    }
    pub fn intersects(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let a = ray.direction().x().powi(2) + ray.direction().z().powi(2);
        if a.approx_eq(0.0) {
            // ray is parallel to the y axis
            return Intersections::new();
        }
        let b = 2.0 * ray.origin().x() * ray.direction().x()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c = ray.origin().x().powi(2) + ray.origin().z().powi(2) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new();
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };
        let mut xs = Intersections::new();
        let y0 = ray.origin().y() + t0 * ray.direction().y();
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(object, t0);
        }

        let y1 = ray.origin().y() + t1 * ray.direction().y();
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(object, t1);
        }
        xs
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        Vector::new(object_point.x(), 0.0, object_point.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Point, Vector};
    #[test]
    fn ray_misses_cylinder() {
        let cyl_obj = Object::new_cylinder(-f64::INFINITY, f64::INFINITY);
        let test_cases = vec![
            (Point::new(1.0, 1.0, 1.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
        ];
        for (origin, direction) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let xs = cyl_obj.intersect(&ray);
            assert_eq!(xs.count(), 0);
        }
    }

    #[test]
    fn ray_strikes_cylinder() {
        let cyl_obj = Object::new_cylinder(-f64::INFINITY, f64::INFINITY);
        let test_cases = vec![
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];
        for (origin, direction, t0, t1) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let xs = cyl_obj.intersect(&ray);
            assert_eq!(xs.count(), 2);
            assert!(xs[0].t().approx_eq_low_precision(t0));
            assert!(xs[1].t().approx_eq_low_precision(t1));
        }
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let cyl_obj = Object::new_cylinder(f64::NEG_INFINITY, f64::INFINITY);
        let test_cases = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in test_cases {
            let n = cyl_obj.normal_at(&point);
            assert_eq!(n, normal);
        }
    }

    #[test]
    fn intersecting_a_constrained_cylinder_with_caps() {
        let cyl_obj = Object::new_cylinder(1.0, 2.0);
        let test_cases = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.5, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];
        for (origin, direction, count) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let xs = cyl_obj.intersect(&ray);
            assert_eq!(xs.count(), count);
        }
    }
}
