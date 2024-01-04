use crate::{rtc::shape::Shape, primitives::{Matrix, Vector, Point, Tuple}};

use super::{ray::Ray, intersection::Intersections}; 
#[derive(Debug)]
pub struct Object{
    shape: Shape,
    transform: Matrix,
}

impl<'a> Object{
    pub fn new_sphere() -> Self{
        Object{
            shape: Shape::Sphere,
            ..Default::default()
        }
    }

    pub fn intersect(&'a self, ray: &'a Ray) -> Intersections {
        self.shape.intersect(ray, self)
    }

    pub fn set_transform(mut self, transform: Matrix) -> Self{
        self.transform = transform;
        self
    }
}

impl PartialEq for Object{
    fn eq(&self, other: &Self) -> bool{
        self.shape == other.shape
    }
}

impl Default for Object{
    fn default() -> Self{
        Object{
        shape: Shape::Sphere,
        transform: Matrix::id()
    }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn intersection(){
        let ray = Ray::new(Point::new(0.0,0.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);  
        assert_eq!(intersections[0].object(), &sphere);
        assert_eq!(intersections[1].object(), &sphere);
    }

    #[test]
    fn tangent_intersection(){
        let ray = Ray::new(Point::new(0.0,1.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);  
        assert_eq!(intersections[0].t(), 5.0);
        assert_eq!(intersections[1].t(), 5.0);
    }

    #[test]
    fn ray_miss(){
        let ray = Ray::new(Point::new(0.0,2.0,-5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 0);  
    }

    #[test]
    fn ray_inside_sphere(){
        let ray = Ray::new(Point::new(0.0,0.0,0.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);  
        assert_eq!(intersections[0].t(), -1.0);
        assert_eq!(intersections[1].t(), 1.0);
    }

    #[test]
    fn sphere_behind_ray(){

        let ray = Ray::new(Point::new(0.0,0.0,5.0),Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::new_sphere();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);  
        assert_eq!(intersections[0].t(), -6.0);
        assert_eq!(intersections[1].t(), -4.0);
    }

    #[test]
    fn default_sphere(){
        let sphere = Object::new_sphere();
        assert_eq!(sphere.transform, Matrix::id());
    }

    #[test]
    fn change_sphere_transform(){
        let mut sphere = Object::new_sphere();
        let transform = Matrix::id().translate(2.0, 3.0, 4.0);
        sphere = sphere.set_transform(transform);
        assert_eq!(sphere.transform, transform);
    }
}