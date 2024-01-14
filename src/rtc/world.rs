use crate::rtc::{object::Object, light::PointLight};
use crate::primitives::{Color, Point, Matrix, Tuple};

use super::intersection::{Intersections, Intersection};
use super::material::Material;
use super::ray::Ray;

pub struct World{
    objects: Vec<Object>,
    light: Vec<PointLight>,
}

impl<'a> World{
    fn new() -> World{
        World{
            objects: Vec::new(),
            light: Vec::new(),
        }
    }

    pub fn objects(&self) -> &Vec<Object>{
        &self.objects
    }

    pub fn intersect(&'a self, ray: &Ray) -> Intersections<'a>{
        let mut intersections:Vec<Intersection<'a>> = vec![];
        for object in &self.objects{
            intersections.append(&mut object.intersect(ray).into_iter().collect())
        }
        Intersections::new().with_intersections(intersections).sort()
    }
}

impl Default for World{
    fn default() -> Self{
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
        let mut s1 = Object::new_sphere();
        s1 = s1.set_material(&Material::new().with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7).with_specular(0.2));
        let mut s2 = Object::new_sphere();
        s2 = s2.set_transform(&Matrix::id().scale(0.5, 0.5, 0.5));
        World{
            objects: vec![s1, s2],
            light: vec![light],
        }

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    use crate::primitives::Vector;
    #[test]
    fn test_world(){
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light.len(), 0);
    }

    #[test]
    fn test_default_world(){
        let w = World::default();
        assert_eq!(w.light[0], PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)));
        assert_eq!(w.objects[0].material().color(), Color::new(0.8, 1.0, 0.6));
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.light.len(), 1);
    }

    #[test]
    fn intersect_world_with_ray(){
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }
}
