use ray_tracer::{
   primitives::{
        Canvas, Color,Point,  Tuple
    },
    rtc::{
        ray::Ray,
        object::Object, material::Material, light::PointLight,
    }
};

fn main(){
    let origin = Point::new(0.0,0.0,-5.0);
    let mut canvas = Canvas::new(500, 500);
    let wall_z = 9.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size/ canvas_pixels as f64;
    let half = wall_size/ 2.0;
    let light_position = Point::new(-10.0, -10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, light_position);
    let sphere = Object::new_sphere().set_material(&Material::new().with_color(Color::new(1.0, 0.2, 1.0)));
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0 .. canvas_pixels {
            let world_x = -half + x as f64 * pixel_size;
            let position = Point::new(world_x, world_y,wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            let xs = sphere.intersect(&ray);
            if let Some(hit) = xs.hit(){
                let point = ray.position(hit.t());
                let normal = hit.object().normal_at(&point);
                let eye = -ray.direction();
                let color = hit.object().material().lighting(&light, &point,&point, &eye, &normal, false);
                canvas.write_pixel(y, x, color);
            }

        }
    }
    canvas.save_as_ppm("samples/chapter_5").unwrap();
}
