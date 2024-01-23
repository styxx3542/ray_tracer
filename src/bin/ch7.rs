use ray_tracer::{
    primitives::{Color, Matrix, Point, Tuple, Vector},
    rtc::{
        camera::Camera, light::PointLight, material::Material, object::Object,
        transformation::view_transform, world::World,
    },
};

fn main() {
    let floor = Object::new_plane().set_material(
        &Material::new()
            .with_color(Color::new(1.0, 0.9, 0.0))
            .with_specular(0.0),
    );
    let middle = Object::new_sphere()
        .set_transform(&Matrix::id().translate(-0.5, 1.0, 0.5))
        .set_material(
            &Material::new()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let left = Object::new_sphere()
        .set_transform(
            &Matrix::id()
                .scale(0.33, 0.33, 0.33)
                .translate(-1.5, 0.33, -0.75),
        )
        .set_material(
            &Material::new()
                .with_color(Color::new(1.0, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let light_source = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
    let world = World::new()
        .with_objects(vec![floor, middle, left])
        .with_lights(vec![light_source]);
    let camera = Camera::new(
        2000,
        1000,
        std::f64::consts::FRAC_PI_3,
        view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("samples/chapter_8").unwrap();
}
