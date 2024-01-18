use crate::primitives::{Matrix, Point, Tuple, Canvas};
use crate::rtc::{ray::Ray, world::World};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    transform_inverse: Matrix, // caching the inverse of the transform matrix
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Matrix) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            transform_inverse: transform.inverse().unwrap(),
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / (hsize as f64),
        }
    }
    
    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform_inverse * Point::new(world_x, world_y, -1.0);
        let origin = self.transform_inverse * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float::ApproxEq;
    use crate::primitives::{Vector, Color};
    use crate::rtc::transformation::view_transform;
    #[test]
    fn test_camera() {
        let c = Camera::new(160, 120, std::f64::consts::PI / 2.0, Matrix::id());
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, std::f64::consts::PI / 2.0);
        assert_eq!(c.transform, Matrix::id());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::PI / 2.0, Matrix::id());
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas(){
        let c = Camera::new(125, 200, std::f64::consts::PI / 2.0, Matrix::id());
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0, Matrix::id());
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0, Matrix::id());
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, std::f64::consts::PI / 2.0, Matrix::id());
        c.transform = Matrix::id().rotate_y(std::f64::consts::PI / 4.0) * Matrix::id().translate(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::new(0.0, 2.0, -5.0));
        assert_eq!(r.direction(), Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0));
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, std::f64::consts::PI / 2.0, Matrix::id());
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);
        let image = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
