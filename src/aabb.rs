use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;

pub struct AABB {
    minimum: Point3,
    maximum: Point3
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn min(&self) -> &Point3 {
        &self.minimum
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> bool {
        hit_axis(self.min().x(), self.max().x(), r.origin().x(), r.direction().x(), t_min, t_max) &&
        hit_axis(self.min().y(), self.max().y(), r.origin().y(), r.direction().y(), t_min, t_max) &&
        hit_axis(self.min().z(), self.max().z(), r.origin().z(), r.direction().z(), t_min, t_max)
    }

    pub fn joint_box(box0: &AABB, box1: &AABB) -> AABB {
        let min_x = box0.min().x().min(box1.min().x());
        let min_y = box0.min().y().min(box1.min().y());
        let min_z = box0.min().z().min(box1.min().z());
        let max_x = box0.max().x().max(box1.max().x());
        let max_y = box0.max().y().max(box1.max().y());
        let max_z = box0.max().z().max(box1.max().z());

        AABB {
            minimum: Vec3::new(min_x, min_y, min_z),
            maximum: Vec3::new(max_x, max_y, max_z)
        }
    }

    pub fn copy(&self) -> Self {
        Self::new(self.minimum.copy(), self.maximum.copy())
    }
}

fn hit_axis(min_axis: Float, max_axis: Float,
            ray_orig_axis: Float, ray_dir_axis: Float,
            t_min: Float, t_max: Float) -> bool {
    let inv_d = ray_dir_axis.recip();
    let mut t0 = (min_axis - ray_orig_axis) * inv_d;
    let mut t1 = (max_axis - ray_orig_axis) * inv_d;
    if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
    }
    let t_min = if t0 > t_min { t0 } else { t_min };
    let t_max = if t1 < t_max { t1 } else { t_max };

    t_max > t_min
}
