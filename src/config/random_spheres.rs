//! configuration for the random spheres scene

use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::config::SceneConfig;
use crate::object::material::Dielectric;
use crate::object::texture::{CheckerTexture, SolidColor};
use crate::object::{
    make_bouncing_sphere, make_material, make_sphere, LambertianDiffuse, Metal, World,
};
use crate::render::Camera;
use crate::utils::Vec3;

pub struct RandomSpheresScene {
    pub bounce: bool,
}

impl SceneConfig for RandomSpheresScene {
    // configure the camera
    fn get_camera(&self) -> Camera {
        let look_from = Vec3::new(13.0, 2.0, 4.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        Camera::look_from(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            1.5,
            0.0,
            (look_at - look_from).length(),
            0.0,
            0.25,
        )
    }

    // configure the random sphere scene
    fn get_world(&self) -> World {
        let mut world = World::new();

        // checkered ground
        let mat_ground = make_material(LambertianDiffuse {
            texture: Arc::new(CheckerTexture {
                odd_color: Arc::new(SolidColor::new(1.0, 1.0, 1.0)),
                even_color: Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
            }),
        });
        let sphere_ground = make_sphere(Vec3::new(0.0, -1000.0, -1.0), 1000.0, &mat_ground);
        world.add_hittable(&sphere_ground);

        let mut rng = thread_rng();
        for i in -11..=11 {
            for j in -11..=11 {
                if j == 0 {
                    continue;
                }
                let center = Vec3::new(
                    i as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                    0.3,
                    j as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                );
                let rand = rng.gen::<f64>();
                let m = if rand < 0.65 {
                    make_material(LambertianDiffuse {
                        texture: Arc::new(SolidColor::random()),
                    })
                } else if rand < 0.9 {
                    make_material(Metal {
                        fuzziness: rng.gen_range(0.0, 0.5),
                        albedo: Vec3::random(0.5, 1.0),
                    })
                } else {
                    make_material(Dielectric::new(1.33, Vec3::one()))
                };

                let b = if self.bounce && m.get_type() == "LambertianDiffuse" {
                    make_bouncing_sphere(center, 0.3, rng.gen_range(0.0, 1.0), 0.0, 0.5, &m)
                } else {
                    make_sphere(center, 0.3, &m)
                };
                world.add_hittable(&b);
            }
        }

        // add three giant balls!
        let m1 = make_material(LambertianDiffuse {
            texture: Arc::new(SolidColor::random()),
        });
        let m2 = make_material(Dielectric::new(1.33, Vec3::one()));
        let m3 = make_material(Metal {
            fuzziness: 0.1,
            albedo: Vec3::new(0.7, 0.6, 0.5),
        });
        let b1 = make_sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, &m1);
        let b2 = make_sphere(Vec3::new(0.0, 1.0, 0.0), 1.0, &m2);
        let b3 = make_sphere(Vec3::new(4.0, 1.0, 0.0), 1.0, &m3);
        world.add_hittable(&b1);
        world.add_hittable(&b2);
        world.add_hittable(&b3);
        world.update_metadata();
        world
    }
}
