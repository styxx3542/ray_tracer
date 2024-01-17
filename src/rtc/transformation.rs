use crate::primitives::{Point, Vector, Matrix, Tuple};

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross_product(up.normalize());
    let true_up = left.cross_product(forward);
    let mut orientation = Matrix::id();
    orientation[(0,0)] = left.x();
    orientation[(0,1)] = left.y();
    orientation[(0,2)] = left.z();
    orientation[(1,0)] = true_up.x();
    orientation[(1,1)] = true_up.y();
    orientation[(1,2)] = true_up.z();
    orientation[(2,0)] = -forward.x();
    orientation[(2,1)] = -forward.y();
    orientation[(2,2)] = -forward.z();
    orientation * Matrix::id().translate(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn transformation_matrix_for_default_orientation(){
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix::id());
    }

    #[test]
    fn view_transformation_in_positive_x(){
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix::id().scale(-1.0, 1.0, -1.0));
    }

    #[test]
    fn arbitrary_view_transformation(){
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let expected = Matrix::from_array([-0.50709, 0.50709, 0.67612, -2.36643,
                                             0.76772, 0.60609, 0.12122, -2.82843,
                                             -0.35857, 0.59761, -0.71714, 0.00000,
                                             0.00000, 0.00000, 0.00000, 1.00000]);
        assert_eq!(t, expected);
    }


}
