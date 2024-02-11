use ray_tracer::{
    primitives::{Color, Matrix, Point, Tuple, Vector},
    rtc::{
        camera::Camera, light::PointLight, material::Material, object::Object, pattern::Pattern,
        transformation::view_transform, world::World,
    },
};

fn main() {
    let floor = Object::new_plane().set_material(
        &Material::new()
            .with_pattern(Pattern::new_checkers(
                Color::white(),
                Color::new(0.5, 0.5, 0.5),
            ))
            .with_reflective(0.0),
    );

    let left_wall = Object::new_plane()
        .set_transform(
            &Matrix::id()
                .rotate_z(std::f64::consts::FRAC_PI_2)
                .translate(-15.0, 0.0, 0.0)
        )
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_checkers(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.0),
        );

    let right_wall = Object::new_plane()
        .set_transform(
            &Matrix::id()
                .rotate_x(std::f64::consts::FRAC_PI_2)
                .translate(0.0, 0.0, 15.0)
        )
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_checkers(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.0),
        );
    let blue_sphere = Object::new_sphere()
        .set_transform(&Matrix::id().translate(-8.0, 1.0, 5.0))
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_gradient(
                    Color::new(0.0, 0.0, 1.0),
                    Color::new(0.0, 0.0, 0.0),
                ))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        );

    let refractive_sphere = Object::new_sphere()
        .set_transform(&Matrix::id().translate(0.0, 1.5, 0.0))
        .set_material(
            &Material::new()
                .with_color(Color::new(0.1, 0.1, 0.1))
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );

    let red_sphere = Object::new_sphere()
        .set_transform(&Matrix::id().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, 5.0))
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_gradient(
                    Color::new(1.0, 0.0, 0.0),
                    Color::new(0.0, 0.0, 0.0),
                ))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        );
    let green_sphere = Object::new_sphere()
        .set_transform(
            &Matrix::id()
                .scale(0.5, 0.5, 0.5)
                .translate(-2.3, 0.5, -0.77),
        )
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_gradient(
                    Color::new(0.0, 1.0, 0.0),
                    Color::new(0.0, 0.0, 0.0),
                ))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        );

    let light_source = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(5.0, 10.0, -10.0));
    let world = World::new()
        .with_objects(vec![
            green_sphere,
            blue_sphere,
            floor,
            red_sphere,
            refractive_sphere,
            left_wall,
            right_wall
        ])
        .with_lights(vec![light_source]);
    let camera = Camera::new(
        2000,
        1000,
        std::f64::consts::PI / 3.0,
        view_transform(
            Point::new(5.0, 1.5, -5.5),
            Point::new(0.0, 0.7, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("samples/chapter_8").unwrap();
}
