use rand::{thread_rng, Rng};

use crate::object::{Face, HitRecord, TextureObject};
use crate::utils::{Color, Ray, Vec3};

pub struct FilteredRay {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn get_type(&self) -> &'static str;
    fn emit(&self, _u: f64, _v: f64, _p: Vec3<f64>) -> Color {
        Color::zero()
    }
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay>;
}

pub struct LambertianDiffuse {
    pub texture: TextureObject,
}

// helper function
fn rand_unit_vector() -> Vec3<f64> {
    let mut r = thread_rng();
    let a: f64 = r.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z: f64 = r.gen_range(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

impl Material for LambertianDiffuse {
    fn get_type(&self) -> &'static str {
        "LambertianDiffuse"
    }
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        Some(FilteredRay {
            attenuation: self.texture.get_color(h.u, h.v, h.p),
            scattered: Ray {
                orig: h.p,
                dir: h.normal + rand_unit_vector(),
                t: r.time(),
            },
        })
    }
}

pub struct Metal {
    pub fuzziness: f64,
    pub albedo: Vec3<f32>,
}

impl Metal {
    pub fn new(fuzziness: f64, albedo: Vec3<f32>) -> Metal {
        if fuzziness > 1.0 {
            return Metal {
                fuzziness: 1.0,
                albedo,
            };
        } else {
            return Metal { fuzziness, albedo };
        }
    }
}

impl Material for Metal {
    fn get_type(&self) -> &'static str {
        "Metal"
    }
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        let reflect_dir = r.direction() - h.normal * (2.0 * r.direction().dot(h.normal))
            + rand_unit_vector() * self.fuzziness;
        return if reflect_dir.dot(h.normal) > 0.0 {
            Some(FilteredRay {
                attenuation: self.albedo,
                scattered: Ray {
                    orig: h.p,
                    dir: reflect_dir,
                    t: r.time(),
                },
            })
        } else {
            None
        };
    }
}

pub struct Dielectric {
    pub eta: f64,
    eta_inv: f64,
    r0: f64,
    pub albedo: Vec3<f32>,
}

impl Dielectric {
    pub fn new(eta: f64, albedo: Vec3<f32>) -> Dielectric {
        let mut r0 = (1.0 - eta) / (1.0 + eta);
        r0 = r0 * r0;
        Dielectric {
            eta,
            eta_inv: 1.0 / eta,
            r0,
            albedo,
        }
    }

    // approximation for reflection probability
    #[inline(always)]
    fn schlick(&self, cosine: f64) -> f64 {
        self.r0 + (1.0 - self.r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn get_type(&self) -> &'static str {
        "Dielectric"
    }
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        let er = match h.f {
            Face::Inward => self.eta_inv,
            Face::Outward => self.eta,
        };
        let ru = r.direction().unit_vector();
        let cos_theta = -ru.dot(h.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let rnd: f64 = thread_rng().gen();
        let dir = if sin_theta * er > 1.0 || rnd < self.schlick(cos_theta) {
            // reflect
            ru - h.normal * (2.0 * cos_theta)
        } else {
            // refract
            let r_parallel = (ru + h.normal * cos_theta) * er;
            let r_perp = h.normal * (-((1.0 - r_parallel.length_square()).sqrt()));
            r_parallel + r_perp
        };
        Some(FilteredRay {
            attenuation: self.albedo,
            scattered: Ray {
                orig: h.p,
                dir,
                t: r.time(),
            },
        })
    }
}

pub struct DiffuseLight {
    pub emit: TextureObject,
    pub brightness: f32,
}

impl Material for DiffuseLight {
    fn get_type(&self) -> &'static str {
        "DiffuseLight"
    }

    fn emit(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        self.emit.get_color(u, v, p) * self.brightness
    }

    fn scatter(&self, _r: &Ray, _h: &HitRecord) -> Option<FilteredRay> {
        None
    }
}

pub struct Isotropic {
    pub albedo: TextureObject,
}

impl Material for Isotropic {
    fn get_type(&self) -> &'static str {
        "Isotropic"
    }

    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        Some(FilteredRay {
            attenuation: self.albedo.get_color(h.u, h.v, h.p),
            scattered: Ray {
                orig: h.p,
                dir: rand_unit_vector(),
                t: r.t,
            },
        })
    }
}
