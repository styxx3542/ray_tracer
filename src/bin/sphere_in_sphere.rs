use ray_tracer::{
    primitives::{Color, Matrix, Point, Tuple, Vector},
    rtc::{
        camera::Camera, light::PointLight, material::Material, object::Object, pattern::Pattern,
        transformation::view_transform, world::World,
    },
};

fn main() {
    let wall = Object::new_plane()
        .set_transform(
            &Matrix::id()
                .rotate_x(std::f64::consts::FRAC_PI_2)
                .translate(0.0, 0.0, 10.0),
        )
        .set_material(
            &Material::new()
                .with_pattern(Pattern::new_checkers(
                    Color::new(0.15, 0.15, 0.15),
                    Color::new(0.85, 0.85, 0.85),
                ))
                .with_ambient(0.8)
                .with_diffuse(0.2)
                .with_specular(0.0),
        );

    let outer_sphere = Object::new_sphere().set_material(
        &Material::new()
            .with_diffuse(0.0)
            .with_ambient(0.0)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_transparency(0.9)
            .with_refractive_index(1.5)
            .with_reflective(0.9),
    ).set_transform(&Matrix::id().translate(-2.0, 0.0, 0.0));
    
    let outer_sphere_2 = Object::new_sphere().set_material(
        &Material::new()
            .with_diffuse(0.0)
            .with_ambient(0.0)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_transparency(0.9)
            .with_refractive_index(1.5)
            .with_reflective(0.9),
    ).set_transform(&Matrix::id().translate(2.0, 0.0, 0.0));


    let inner_sphere = Object::new_sphere()
        .set_material(
            &Material::new()
                .with_color(Color::new(1.0, 1.0, 1.0))
                .with_diffuse(0.0)
                .with_ambient(0.0)
                .with_specular(0.9)
                .with_shininess(300.0)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.0000034),
        ).set_transform(&Matrix::id().scale(0.5, 0.5, 0.5).translate(-2.0, 0.0, 0.0));

    let inner_sphere_2 = Object::new_sphere()
        .set_material(
            &Material::new()
                .with_color(Color::new(1.0, 1.0, 1.0))
                .with_diffuse(0.0)
                .with_ambient(0.0)
                .with_specular(0.9)
                .with_shininess(300.0)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.0000034),
        ).set_transform(&Matrix::id().scale(0.5, 0.5, 0.5).translate(2.0, 0.0, 0.0));

    let light_source = PointLight::new(Color::new(0.9, 0.9, 0.9), Point::new(2.0, 10.0, -5.0));
    let world = World::new()
        .with_objects(vec![outer_sphere, inner_sphere,outer_sphere_2, inner_sphere_2, wall])
        .with_lights(vec![light_source]);
    let camera = Camera::new(
        2000,
        2000,
        std::f64::consts::PI / 3.0,
        view_transform(
            Point::new(0.0, 0.0, -8.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("samples/sphere_in_sphere").unwrap();
}
