use ray_tracer::ppm::{Picture, Color};
use ray_tracer::vec::Vec3;
use ray_tracer::ray::Ray;
use ray_tracer::world::{Sphere, World, Hittable};
use std::rc::Rc;
use std::cell::RefCell;
use num_traits::float::FloatCore;
use ray_tracer::render::Camera;
use rand::Rng;

fn ray_color(r: &Ray, w: &World) -> Color {
    const LOW: Color = Color{0: 1.0, 1: 1.0, 2: 1.0};
    const HIGH: Color = Color{0: 0.5, 1: 0.7, 2: 1.0};
    if let Some(t) = w.hit(&r, 0.0, f64::infinity()) {
        let norm = t.normal;
        (norm.unit_vector() + 1.0) * 0.5
    } else {
        let unit = r.direction().unit_vector();
        let t = 0.5 * (unit.y() + 1.0) as f64;
        LOW * (1.0 - t) + HIGH * t
    }
}

fn main() {
    let width = 200;
    let height = 100;

    let mut p = Picture::new(width, height);

    let origin = Vec3(0.0, 0.0, 0.0);
    let viewport_start = Vec3(-2.0, -1.0, -1.0);
    let hlength = 4.0;
    let vlength = 2.0;

    // initialize the world
    let mut world = World::new();
    let camera = Camera::new(origin, viewport_start, hlength, vlength);
    let sphere1: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    }));
    let sphere2: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    }));
    world.add_hittable(&sphere1);
    world.add_hittable(&sphere2);

    let sample_per_pixel = 20;
    let mut rng = rand::thread_rng();

    // neg-y axis is i, pos-x axis is j
    for i in 0..height {
        for j in 0..width {
            let mut c: Color = Color::default();
            for _k in 0..sample_per_pixel {
                let v = (rng.gen::<f64>() + (height - i - 1) as f64) / height as f64;
                let u = (rng.gen::<f64>() + j as f64) / width as f64;
                c += ray_color(&camera.get_ray(u, v), &world);
            }
            c /= sample_per_pixel as f64;
            p.data[(i * width + j) as usize] = c;
        }
    }

    p.write_to_file("out.ppm").expect("Failed to write file.");
}
