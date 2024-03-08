use crate::{
    float::{approx_eq::ApproxEq, epsilon::LOW_EPSILON},
    primitives::{Point, Tuple, Vector},
    rtc::{
        intersection::{Intersection, Intersections},
        object::Object,
        ray::Ray,
    },
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cone {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            closed: false,
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }
}
impl<'a> Cone {
    pub fn new(minimum: f64, maximum: f64, closed: bool) -> Self {
        Cone {
            minimum,
            maximum,
            closed,
        }
    }

    pub fn check_cap(&self, ray: &Ray, t: f64, y: f64) -> bool {
        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        (x.powi(2) + z.powi(2)) <= y.abs() 
    }
    pub fn intersects(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let a =
            ray.direction().x().powi(2) - ray.direction().y().powi(2) + ray.direction().z().powi(2);
        let b = 2.0 * ray.origin().x() * ray.direction().x()
            - 2.0 * ray.origin().y() * ray.direction().y()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c =
            ray.origin().x().powi(2) - ray.origin().y().powi(2) + ray.origin().z().powi(2) ;

        if a.approx_eq(0.0) && b.approx_eq(0.0) {
            // ray is parallel to the cone surface
            return self.intersection_at_caps(ray, object);
        }
        if a.approx_eq(0.0) && !b.approx_eq(0.0) {
            //ray intersects the cone at a single point
            let t = -c / (2.0 * b);
            let mut xs = Intersections::new().with_intersections(vec![Intersection::new(t, object)]);
            xs.extend(self.intersection_at_caps(ray, object));
            return xs;
        }
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
        let intersection_at_caps = self.intersection_at_caps(ray, object);
        xs.extend(intersection_at_caps);
        xs
    }

    fn intersection_at_caps(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if !self.closed || ray.direction().y().approx_eq(0.0) {
            return xs;
        }
        let t0 = (self.minimum - ray.origin().y()) / ray.direction().y();
        if self.check_cap(ray, t0, self.minimum) {
            xs.push(object, t0);
        }
        let t1 = (self.maximum - ray.origin().y()) / ray.direction().y();
        if self.check_cap(ray, t1, self.maximum) {
            xs.push(object, t1);
        }
        xs
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        let dist = object_point.x().powi(2) + object_point.z().powi(2);
        if dist < 1.0 && object_point.y() >= self.maximum - LOW_EPSILON {
            return Vector::new(0.0, 1.0, 0.0);
        } else if dist < 1.0 && object_point.y() <= self.minimum + LOW_EPSILON {
            return Vector::new(0.0, -1.0, 0.0);
        }
        let y = (object_point.x().powi(2) + object_point.z().powi(2)).sqrt();
        let y = if object_point.y() > 0.0 { -y } else { y };
        Vector::new(object_point.x(), y, object_point.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Point, Vector};
    use pretty_assertions::assert_eq;
    #[test]
    fn intersecting_cone_with_ray() {
        let c = Object::new_cone(f64::NEG_INFINITY, f64::INFINITY);
        let intersections = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];
        for (origin, direction, t1, t2) in intersections {
            let r = Ray::new(origin, direction.normalize());
            let xs = c.intersect(&r);
            assert_eq!(xs.count(), 2);
            assert!(xs[0].t().approx_eq_low_precision(t1));
            assert!(xs[1].t().approx_eq_low_precision(t2));
        }
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_half(){
        let c = Object::new_cone(f64::NEG_INFINITY, f64::INFINITY);
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = c.intersect(&r);
        assert_eq!(xs.count(), 1);
        assert!(xs[0].t().approx_eq_low_precision(0.35355));
    }

    #[test]
    fn intersecting_cone_end_caps(){
        let c = Object::new_closed_cone(-0.5, 0.5);
        let intersections = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];
        for (origin, direction, count) in intersections {
            let r = Ray::new(origin, direction.normalize());
            let xs = c.intersect(&r);
            assert_eq!(xs.count(), count);
        }
    }

    #[test]
    fn computing_normal_vector_on_cone(){
        let c = Object::new_cone(f64::NEG_INFINITY, f64::INFINITY);
        let normals = vec![
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, -2.0_f64.sqrt(), 1.0)),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for (origin, normal) in normals {
            let n = c.shape().normal_at(&origin);
            assert_eq!(n, normal);
        }
    }
}
