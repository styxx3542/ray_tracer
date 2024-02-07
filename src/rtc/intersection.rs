use crate::{
    float::{epsilon::EPSILON, ApproxEq},
    primitives::{Point, Vector},
};
use std::{cmp::Ord, cmp::Ordering, cmp::PartialOrd, ops::Index};

use super::{object::Object, ray::Ray};
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Intersection { t, object }
    }
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a Object {
        self.object
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            return Ordering::Less;
        } else if self.t < other.t {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<'a> std::cmp::Eq for Intersection<'a> {}

#[derive(Debug)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Default for Intersections<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Intersections<'a> {
    pub fn new() -> Intersections<'a> {
        Intersections {
            intersections: Vec::<Intersection<'a>>::new(),
        }
    }

    pub fn with_intersections(mut self, intersections: Vec<Intersection<'a>>) -> Self {
        self.intersections = intersections;
        self
    }

    pub fn push(&mut self, object: &'a Object, t: f64) {
        self.intersections.push(Intersection::new(t, object))
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Intersection<'a>> {
        self.intersections.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Intersection<'a>> {
        self.intersections.into_iter()
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.iter().find(|i| i.t() >= 0.0)
    }

    pub fn sort(mut self) -> Intersections<'a> {
        self.intersections.sort_unstable();
        self
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

pub struct IntersectionState<'a> {
    t: f64,
    object: &'a Object,
    eyev: Vector,
    point: Point,
    normalv: Vector,
    inside: bool,
    over_point: Point,
    reflectv: Vector,
    n1: f64,
    n2: f64,
}
#[derive(Debug)]
struct RefractionState {
    n1: f64,
    n2: f64,
    is_entering: bool,
}

fn calculate_refraction_state(ray: &Ray, intersection: &Intersection) -> RefractionState {
    let current_index = intersection.object().material().refractive_index();
    let objects = ray.get_indices();
    let is_entering = (*objects)
        .iter()
        .find(|o| (*o).approx_eq(current_index))
        .is_none();
    let previous_refraction_index: f64  = *objects
        .last()
        .unwrap_or(&1.0);
    if is_entering {
        return RefractionState {
            n1: previous_refraction_index,
            n2: current_index,
            is_entering: true,
        };
    }
    let prev = objects.iter().rev().find(|o| !(*o).approx_eq(current_index));
    let outer_refraction_index = prev.unwrap_or(&1.0);

    RefractionState {
        n1: previous_refraction_index,
        n2: *outer_refraction_index,
        is_entering: false,
    }
}

impl<'a> IntersectionState<'a> {
    pub fn new(
        t: f64,
        object: &'a Object,
        eyev: Vector,
        point: Point,
        normalv: Vector,
        inside: bool,
        over_point: Point,
        reflectv: Vector,
        n1: f64,
        n2: f64,
    ) -> Self {
        IntersectionState {
            t,
            object,
            eyev,
            point,
            normalv,
            inside,
            over_point,
            reflectv,
            n1,
            n2,
        }
    }

    pub fn prepare_computations(
        intersection: &'a Intersection,
        ray: &mut Ray,
    ) -> IntersectionState<'a> {
        let t = intersection.t();
        let state = calculate_refraction_state(ray, intersection);
        if state.is_entering {
            ray.add_index(intersection.object().material().refractive_index());
        } else {
            ray.remove_index(intersection.object().material().refractive_index());
        }
        let object = intersection.object();
        let point = ray.position(t);
        let eyev = -ray.direction();
        let normalv = object.normal_at(&point);
        let (normalv, inside) = {
            if normalv.dot_product(eyev) < 0.0 {
                (-normalv, true)
            } else {
                (normalv, false)
            }
        };
        let over_point = point + normalv * EPSILON;
        let reflectv = ray.direction().reflect(&normalv);
        IntersectionState::new(
            t, object, eyev, point, normalv, inside, over_point, reflectv, state.n1, state.n2,
        )
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a Object {
        self.object
    }

    pub fn eyev(&self) -> Vector {
        self.eyev
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn normalv(&self) -> Vector {
        self.normalv
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn reflectv(&self) -> Vector {
        self.reflectv
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        float::ApproxEq,
        primitives::{Matrix, Tuple},
        rtc::{intersection::Intersection, material::Material},
    };
    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2]);
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2]);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new()
            .with_intersections(vec![i1.clone(), i2, i3.clone(), i4.clone()])
            .sort();
        assert_eq!(xs.hit(), Some(&i4));
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = IntersectionState::prepare_computations(&i, &mut r);
        assert_eq!(comps.t(), i.t());
        assert_eq!(comps.object(), i.object());
        assert_eq!(comps.point(), Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev(), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = IntersectionState::prepare_computations(&i, &mut r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let i = Intersection::new(1.0, &shape);
        let comps = IntersectionState::prepare_computations(&i, &mut r);
        assert_eq!(comps.point(), Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev(), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn precompute_reflection_vector() {
        let shape = Object::new_plane();
        let mut r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = IntersectionState::prepare_computations(&i, &mut r);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn check_refractive_indices() {
        let a = Object::new_glass_sphere()
            .set_transform(&Matrix::id().scale(2.0, 2.0, 2.0))
            .set_material(&Material::new().with_refractive_index(1.5));
        let b = Object::new_glass_sphere()
            .set_transform(&Matrix::id().translate(0.0, 0.0, -0.25))
            .set_material(&Material::new().with_refractive_index(2.0));
        let c = Object::new_glass_sphere()
            .set_transform(&Matrix::id().translate(0.0, 0.0, 0.25))
            .set_material(&Material::new().with_refractive_index(2.5));
        let mut r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ]);
        let indices = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for (i, (n1, n2)) in indices.iter().enumerate() {
            let comps = IntersectionState::prepare_computations(&xs[i], &mut r);
            println!("{}, {} {}, {}",  n1, n2, comps.n1, comps.n2);
            assert!(comps.n1.approx_eq(*n1));
            assert!(comps.n2.approx_eq(*n2));
        }
    }
}
