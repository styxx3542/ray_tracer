use crate::{rtc::{intersection::{Intersections, Intersection}, object::Object, ray::Ray}, primitives::{Vector, Point}};
use crate::primitives::Tuple;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cube{
}

impl<'a> Cube{
    fn check_axis(origin: f64, direction: f64) -> (f64, f64){
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let (tmin, tmax) = if direction.abs() >= 1e-5{
            (tmin_numerator / direction, tmax_numerator / direction)
        }else{
            (tmin_numerator * f64::INFINITY, tmax_numerator * f64::INFINITY)
        };
        if tmin > tmax{
            (tmax, tmin)
        }else{
            (tmin, tmax)
        }
    }
    pub fn intersects(ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let ray = ray.transform(&object.transform_inverse());
        let (xtmin, xtmax) = Self::check_axis(ray.origin().x(), ray.direction().x());
        let (ytmin, ytmax) = Self::check_axis(ray.origin().y(), ray.direction().y());
        let (ztmin, ztmax) = Self::check_axis(ray.origin().z(), ray.direction().z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return Intersections::new();
        }

        Intersections::new().with_intersections(vec![Intersection::new(tmin, object),Intersection::new(tmax, object)])
    }

    pub fn normal_at(point: &Point) -> Vector{
        let maxc = point.x().abs().max(point.y().abs()).max(point.z().abs());
        if maxc == point.x().abs(){
            Vector::new(point.x(), 0.0, 0.0)
        }else if maxc == point.y().abs(){
            Vector::new(0.0, point.y(), 0.0)
        }else{
            Vector::new(0.0, 0.0, point.z())
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::primitives::{Point, Vector};
    #[test]
    fn ray_intersects_cube(){
        let intersections = vec![
            (Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0), 4.0, 6.0),
            (Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0), 4.0, 6.0),
            (Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0), 4.0, 6.0),
            (Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0), 4.0, 6.0),
            (Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0), 4.0, 6.0),
            (Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 4.0, 6.0),
            (Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0), -1.0, 1.0),
        ];
        let c = Object::new_cube();
        for (origin, direction, t1, t2) in intersections{
            let r = Ray::new(origin, direction);
            let xs = Cube::intersects(&r, &c);
            assert_eq!(xs.count(), 2);
            assert_eq!(xs[0].t(), t1);
            assert_eq!(xs[1].t(), t2);
        }
    }

    #[test]
    fn ray_misses_cube(){
        let c = Object::new_cube();
        let intersections = vec![
            (Point::new(-2.0, 0.0, 0.0), Vector::new(0.2673, 0.5345, 0.8018)),
            (Point::new(0.0, -2.0, 0.0), Vector::new(0.8018, 0.2673, 0.5345)),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.5345, 0.8018, 0.2673)),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (origin, direction) in intersections{
            let r = Ray::new(origin, direction);
            let xs = Cube::intersects(&r, &c);
            assert_eq!(xs.count(), 0);
        }
    }

    #[test]
    fn normal_on_surface_of_cube(){
        let normals = vec![
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in normals{
            let n = Cube::normal_at(&point);
            assert_eq!(n, normal);
        }
    }
}
