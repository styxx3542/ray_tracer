use std::{cmp::Ord, cmp::Ordering, cmp::PartialOrd, ops::Index};
use super::object::Object;
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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn hit_when_all_intersections_have_positive_t(){
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2]);
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t(){
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t(){
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2]);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection(){
        let s = Object::new_sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2, i3.clone(), i4.clone()]).sort();
        assert_eq!(xs.hit(), Some(&i4));
    }

}
