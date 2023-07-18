use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
type Point3 = Vec3;

use std::mem::MaybeUninit;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT]
}

impl Perlin {
    pub fn new(rng: &mut RngGen) -> Self {
        let mut ranvec_uninit: [MaybeUninit<Vec3>; POINT_COUNT] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for v in ranvec_uninit.iter_mut() {
            let r = Vec3::random_range(rng, -1.0, 1.0).unit_vector();
            *v = MaybeUninit::new(r);
        }
        let mut new = Self {
            ranvec: unsafe {
                std::mem::transmute::<_, [Vec3; POINT_COUNT]>(ranvec_uninit)
            },
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT]
        };
        populate(&mut new.perm_x);
        permute(&mut new.perm_x, rng);
        populate(&mut new.perm_y);
        permute(&mut new.perm_y, rng);
        populate(&mut new.perm_z);
        permute(&mut new.perm_z, rng);
        new
    }

    pub fn noise(&self, p: &Point3) -> Float {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = (p.x().floor() as i32 & 255) as usize;
        let j = (p.y().floor() as i32 & 255) as usize;
        let k = (p.z().floor() as i32 & 255) as usize;
        let mut c: [MaybeUninit<Vec3>; 8] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[dk + (dj<<1) + (di<<2)] = MaybeUninit::new(self.ranvec[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ].copy());
                }
            }
        }

        let c = unsafe { std::mem::transmute::<_, [Vec3; 8]>(c) };
        perlin_interp(&c, u, v, w)
    }
}

fn populate(p: &mut [usize; POINT_COUNT]) {
    for i in 0..POINT_COUNT {
        p[i] = i;
    }
}

fn permute(p: &mut [usize; POINT_COUNT], rng: &mut RngGen) {
    for i in (1..POINT_COUNT).rev() {
        let target = rng.usize(0, i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: &[Vec3; 8], u: Float, v: Float, w: Float) -> Float {
    let uu = u*u*(3.0-2.0*u);
    let vv = v*v*(3.0-2.0*v);
    let ww = w*w*(3.0-2.0*w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as Float, v - j as Float, w - k as Float);
                accum += (i as Float * uu + (1.0-i as Float)*(1.0-uu))*
                        (j as Float * vv + (1.0-j as Float)*(1.0-vv))*
                        (k as Float * ww + (1.0-k as Float)*(1.0-ww))*
                        Vec3::dot(&c[k + (j<<1) + (i<<2)], &weight_v);
            }
        }
    }
    accum
}
