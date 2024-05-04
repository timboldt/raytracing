use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest = ray_tmax;

        for object in self {
            if let Some(rec) = object.hit(ray, ray_tmin, closest) {
                closest = rec.t;
                hit_rec = Some(rec);
            }
        }

        hit_rec
    }
}