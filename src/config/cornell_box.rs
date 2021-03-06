use std::sync::Arc;

use crate::config::SceneConfig;
use crate::object::constant_medium::ConstantMedium;
use crate::object::cube::Cube;
use crate::object::material::DiffuseLight;
use crate::object::rect::{XYRect, XZRect, YZRect};
use crate::object::rotate::RotateY;
use crate::object::{
    make_hittable, make_material, make_texture, LambertianDiffuse, SolidColor, World,
};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::render::Camera;
use crate::utils::{Color, Vec3};

pub struct CornellBoxScene {}

impl SceneConfig for CornellBoxScene {
    fn get_camera(&self) -> Camera {
        let look_from = Vec3::new(273.0, 273.0, 1300.0);
        let look_at = Vec3::new(273.0, 273.0, 0.0);
        Camera::look_from(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            1.0,
            0.0,
            (look_at - look_from).length(),
            0.0,
            0.01,
        )
    }

    fn get_world(&self) -> World {
        let red = make_material(LambertianDiffuse {
            texture: make_texture(SolidColor::new(0.65, 0.05, 0.05)),
        });
        let white = make_material(LambertianDiffuse {
            texture: make_texture(SolidColor::new(0.73, 0.73, 0.73)),
        });
        let green = make_material(LambertianDiffuse {
            texture: make_texture(SolidColor::new(0.12, 0.45, 0.15)),
        });
        let light = make_material(DiffuseLight {
            emit: make_texture(SolidColor::new(1.0, 1.0, 1.0)),
            brightness: 15.0,
        });
        let mut world = World::new();
        // generate walls
        let left = make_hittable(YZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, &green));
        let right = make_hittable(YZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, &red));
        let up = make_hittable(XZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, &white));
        let down = make_hittable(XZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, &white));
        let behind = make_hittable(XYRect::new((0.0, 0.0), (555.0, 555.0), 0.0, &white));
        let lamp = make_hittable(XZRect::new((213.0, 227.0), (343.0, 332.0), 554.0, &light));

        let c1 = make_hittable(Cube::new(
            Vec3::new(265.0, 0.0, 295.0),
            Vec3::new(430.0, 165.0, 460.0),
            &white,
        ));
        let c2 = make_hittable(Cube::new(
            Vec3::new(130.0, 0.0, 100.0),
            Vec3::new(295.0, 330.0, 300.0),
            &white,
        ));
        let rc1 = make_hittable(RotateY::new(&c1, -5.0));
        let rc2 = make_hittable(RotateY::new(&c2, 10.0));
        let black_color = make_texture(SolidColor::new(0.0, 0.0, 0.0));
        let white_color = make_texture(SolidColor::new(1.0, 1.0, 1.0));
        world.add_hittable(&left);
        world.add_hittable(&right);
        world.add_hittable(&down);
        world.add_hittable(&up);
        world.add_hittable(&behind);
        world.add_hittable(&lamp);
        world.add_hittable(&rc1);
        world.add_hittable(&rc2);

        // set up a dark skybox
        let sb: Arc<dyn SkyBox + Send + Sync> = Arc::new(ColorGradientSkyBox {
            v1: Color::zero(),
            v2: Color::zero(),
        });
        world.set_skybox(&sb);

        world.update_metadata();
        world
    }
}
