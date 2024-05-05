use crate::interval::Interval;
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
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest = interval.clone();

        for object in self {
            if let Some(rec) = object.hit(ray, &closest) {
                closest.max = rec.t;
                hit_rec = Some(rec);
            }
        }

        hit_rec
    }
}