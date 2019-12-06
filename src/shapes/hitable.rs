use crate::util::material::Material;
use crate::util::ray::Ray;
use crate::util::vector3d::Vector3D;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3D,
    pub normal: Vector3D,
    pub material: Material,
}

impl HitRecord {
    pub fn new(m: Material) -> Self {
        Self {
            t: 0.,
            p: Vector3D::new(0., 0., 0.),
            normal: Vector3D::new(0., 0., 0.),
            material: m,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

unsafe impl Sync for HitableList {}
unsafe impl Send for HitableList {}

impl HitableList {
    pub fn new(hitable: Vec<Box<dyn Hitable>>) -> Self {
        Self { list: hitable }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(rec.material.clone());
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
