#![allow(dead_code)]
pub mod primitives {
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;
    mod canvas;
    mod color;
    mod matrix;
    mod matrix2;
    mod matrix3;
    mod point;
    mod tuple;
    mod vector;
}
pub mod rtc {
    pub mod ray;
    pub mod intersection;
    pub mod object;
    pub mod shape;
    pub mod shapes{
        pub mod sphere;
    }
}
mod float {
    pub mod approx_eq;
    pub mod epsilon;
    pub use approx_eq::ApproxEq;
}
