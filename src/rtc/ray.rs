use crate::primitives::{Point, Vector, Matrix};
#[derive(Debug, Clone, Copy)]
pub struct Ray{
    origin: Point,
    direction: Vector,
}
impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray{
        Ray{origin, direction}
    }

    pub fn position(&self, time: f64) -> Point{
        self.origin + self.direction*time
    }

    pub fn origin(&self) -> Point{
        self.origin
    }

    pub fn direction(&self) -> Vector{
        self.direction
    }

    pub fn transform(&self, transform: &Matrix) -> Self{
        Ray::new(*transform * self.origin, *transform * self.direction)
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    use crate::primitives::Tuple;
    #[test]
    fn create_ray(){
        let origin = Point::new(1.0,2.0,3.0);
        let direction = Vector::new(4.0,5.0,6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }
   #[test] 
    fn test_position(){
        let origin = Point::new(2.0,3.0,4.0);
        let direction = Vector::new(1.0,0.0,0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position(0.0), Point::new(2.0,3.0,4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0,3.0,4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0,3.0,4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5,3.0,4.0));
    }

    #[test]
    fn test_transform(){
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::id().translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scale(){
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::id().scale(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
   
}       
