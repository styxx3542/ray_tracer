use ray_tracer::{
    primitives::{Color, Matrix, Point, Tuple, Vector},
    rtc::{light::PointLight, material::Material, object::Object,world::World, camera::Camera, transformation::view_transform},
};

fn main() {
    let floor = Object::new_sphere()
        .set_transform(&Matrix::id().scale(10.0, 0.01, 10.0))
        .set_material(
            &Material::new()
                .with_color(Color::new(1.0, 0.9, 0.9))
                .with_specular(0.0),
        );
    let left_wall = Object::new_sphere()
        .set_transform(
            &Matrix::id()
                .scale(10.0, 0.01, 10.0)
                .rotate_x(std::f64::consts::FRAC_PI_2)
                .rotate_y(-std::f64::consts::FRAC_PI_4)
                .translate(0.0, 0.0, 5.0),
        )
        .set_material(&floor.material());
    let right_wall = Object::new_sphere().set_transform(
        &Matrix::id()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .rotate_y(std::f64::consts::FRAC_PI_4)
            .translate(0.0, 0.0, 5.0),
    );

    let middle = Object::new_sphere()
        .set_transform(&Matrix::id().translate(-0.5, 1.0, 0.5))
        .set_material(
            &Material::new()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let right = Object::new_sphere().set_transform(
        &Matrix::id()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5),
    ).set_material(
        &Material::new()
            .with_color(Color::new(0.5, 1.0, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3),
    );

    let left = Object::new_sphere().set_transform(
        &Matrix::id()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
    ).set_material(
        &Material::new()
            .with_color(Color::new(1.0, 0.8, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3),
    );

    let light_source = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
    let world = World::new().with_objects(vec![
        floor, left_wall, right_wall, middle, right, left,
    ]).with_lights(vec![light_source]);

    let camera = Camera::new(1000, 500, std::f64::consts::FRAC_PI_3, view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas.save_as_ppm("samples/chapter_7").unwrap();
}
