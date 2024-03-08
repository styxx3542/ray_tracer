#![allow(dead_code)]
pub mod primitives {
    pub use canvas::Canvas;
    pub use color::Color;
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;
    pub mod canvas;
    pub mod color;
    mod matrix;
    mod matrix2;
    mod matrix3;
    mod point;
    mod tuple;
    mod vector;
}
pub mod rtc {
    pub mod camera;
    pub mod intersection;
    pub mod light;
    pub mod material;
    pub mod object;
    pub mod ray;
    pub mod shape;
    pub mod transformation;
    pub mod world;
    pub mod pattern;
    pub mod shapes {
        pub mod plane;
        pub mod sphere;
        pub mod cube;
        pub mod cylinder;
        pub mod cone;
    }
}
mod float {
    pub mod approx_eq;
    pub mod epsilon;
    pub use approx_eq::ApproxEq;
}
