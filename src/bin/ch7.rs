use ray_tracer::{
    primitives::{Color, Matrix, Point, Tuple, Vector},
    rtc::{
        camera::Camera, light::PointLight, material::Material, object::Object, pattern::Pattern,
        transformation::view_transform, world::World,
    },
};

fn main() {
    let floor = Object::new_plane().set_material(&Material::new().with_pattern(Pattern::new_checkers(Color::white(), Color::black())).with_reflective(0.3));
    let middle = Object::new_glass_sphere()
        .set_transform(&Matrix::id().translate(-1.3, 1.5, -4.0))
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_gradient(Color::new(0.0, 0.0,1.0), Color::new(0.0, 0.0, 0.0)))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(1.0)
        );
    let left = Object::new_sphere()
        .set_transform(
            &Matrix::id()
                .translate(0.0, 2.0, -6.0),
        )
        .set_material(
            &Material::new()
            .with_transparency(0.5)
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let light_source = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-5.0, 10.0, -10.0));
    let world = World::new()
        .with_objects(vec![left, middle, floor])
        .with_lights(vec![light_source]);
    let camera = Camera::new(
        2000,
        1000,
        std::f64::consts::PI / 1.50,
        view_transform(
            Point::new(-1.0, 2.0, -9.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("samples/chapter_8").unwrap();
}
