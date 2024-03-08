use crate::{
    float::{epsilon::EPSILON, ApproxEq},
    primitives::{Point, Vector},
    rtc::{object::Object, ray::Ray},
};
use std::{cmp::Ord, cmp::Ordering, cmp::PartialOrd, ops::Index};


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

    pub fn extend(&mut self, other: Self) {
        self.intersections.extend(other.intersections);
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
    under_point: Point,
    is_entering: bool,
}
#[derive(Debug)]
struct RefractionState {
    n1: f64,
    n2: f64,
    is_entering: bool,
}

fn calculate_refraction_state(ray: &Ray, intersection: &Intersection) -> RefractionState {
    // Different algorithm for calculating refraction index
    // Store the refraction indices encountered by the ray so far inside the ray in a stack
    // When a ray intersects an object, it checks if it is entering or exiting the objects
    // If it is entering, it pushes the object's refraction index to the stack
    // If it is exiting, it pops the object's refraction index from the stack
    let current_index = intersection.object().material().refractive_index();
    let objects = ray.get_indices();
    let is_entering = (*objects)
        .iter()
        .find(|o| (*o).approx_eq(current_index))
        .is_none();
    let previous_refraction_index: f64 = *objects
        .last()
        .expect("Never should be empty - outside world is always 1.0");
    if is_entering {
        return RefractionState {
            n1: previous_refraction_index,
            n2: current_index,
            is_entering: true,
        };
    }
    let prev = objects
        .iter()
        .rev()
        .find(|o| !(*o).approx_eq(current_index));
    let new_refraction_index = prev.unwrap_or(&previous_refraction_index);

    RefractionState {
        n1: previous_refraction_index,
        n2: *new_refraction_index,
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
        under_point: Point,
        reflectv: Vector,
        n1: f64,
        n2: f64,
        is_entering: bool,
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
            under_point,
            is_entering,
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
            if normalv.dot_product(&eyev) < 0.0 {
                (-normalv, true)
            } else {
                (normalv, false)
            }
        };
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction().reflect(&normalv);

        IntersectionState::new(
            t,
            object,
            eyev,
            point,
            normalv,
            inside,
            over_point,
            under_point,
            reflectv,
            state.n1,
            state.n2,
            state.is_entering,
        )
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev().dot_product(&self.normalv());
        if self.n1 > self.n2{
            let n = self.n1 / self.n2;
            let sin2_t = n*n * (1.0 - cos*cos);
            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)

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

    pub fn n1(&self) -> f64 {
        self.n1
    }

    pub fn n2(&self) -> f64 {
        self.n2
    }

    pub fn under_point(&self) -> Point {
        self.under_point
    }

    pub fn is_entering(&self) -> bool {
        self.is_entering
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
            assert!(comps.n1.approx_eq(*n1));
            assert!(comps.n2.approx_eq(*n2));
        }
    }

    #[test]
    fn under_point_offset_below_surface() {
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape =
            Object::new_glass_sphere().set_transform(&Matrix::id().translate(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = Intersections::new().with_intersections(vec![i]);
        let comps = IntersectionState::prepare_computations(&xs[0], &mut r);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn schlick_under_total_internal_reflection() {
        let shape = Object::new_glass_sphere();
        // ray is coming from inside the glass sphere
        let mut r = Ray::new(Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0), Vector::new(0.0, 1.0, 0.0)).with_indices(vec![1.0, 1.5]);
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, &shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
        ]);
        let comps = IntersectionState::prepare_computations(&xs[1], &mut r);
        let reflectance = comps.schlick();
        assert!(reflectance.approx_eq(1.0));
    } 

    #[test]
    fn schlick_with_perpendicular_viewing_angle() {
        let shape = Object::new_glass_sphere();
        let mut r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ]);
        let comps = IntersectionState::prepare_computations(&xs[1], &mut r);
        let reflectance = comps.schlick();
        assert!(reflectance.approx_eq(0.04));
    }

    #[test]
    fn schlick_with_small_angle_and_n2_greater_than_n1() {
        let shape = Object::new_glass_sphere();
        let mut r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(1.8589, &shape),
        ]);
        let comps = IntersectionState::prepare_computations(&xs[0], &mut r);
        let reflectance = comps.schlick();
        assert!(reflectance.approx_eq_low_precision(0.48873));
    }
}
