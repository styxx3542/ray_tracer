use ray_tracer::{
   primitives::{
        Canvas, Color,Point, Vector, Tuple
    },
    rtc::{
        ray::Ray,
        object::Object,
    }
};

fn main(){
    let origin = Point::new(0.0,0.0,-5.0);
    let mut canvas = Canvas::new(100, 100);
    let color = Color::new(1.0, 0.0, 0.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size/ canvas_pixels as f64;
    let half = wall_size/ 2.0;
    let sphere = Object::new_sphere();
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0 .. canvas_pixels {
            let world_x = -half + x as f64 * pixel_size;
            let position = Point::new(world_x, world_y,wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            let xs = sphere.intersect(&ray);
            if xs.hit().is_some(){
                canvas.write_pixel(y, x, color);
            }

        }
    }
    canvas.save_as_ppm("samples/chapter_5");
}
