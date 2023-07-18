use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
type Point3 = Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: [Float; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT]
}

impl Perlin {
    pub fn new(rng: &mut RngGen) -> Self {
        let mut new = Self {
            ranfloat: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT]
        };
        fill_rand(&mut new.ranfloat, rng);
        populate(&mut new.perm_x);
        permute(&mut new.perm_x, rng);
        populate(&mut new.perm_y);
        permute(&mut new.perm_y, rng);
        populate(&mut new.perm_z);
        permute(&mut new.perm_z, rng);
        new
    }

    pub fn noise(&self, p: &Point3) -> Float {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);

        let i = (p.x().floor() as i32 & 255) as usize;
        let j = (p.y().floor() as i32 & 255) as usize;
        let k = (p.z().floor() as i32 & 255) as usize;

        let mut c: [Float; 8] = [0.0; 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[dk + (dj<<1) + (di<<2)] = self.ranfloat[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
    }
}

fn fill_rand(r: &mut [Float; POINT_COUNT], rng: &mut RngGen) {
    for x in r.iter_mut() {
        *x = rng.get();
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

fn trilinear_interp(c: &[Float; 8], u: Float, v: Float, w: Float) -> Float {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as Float * u + (1.0-i as Float)*(1.0-u))*
                        (j as Float * v + (1.0-j as Float)*(1.0-v))*
                        (k as Float * w + (1.0-k as Float)*(1.0-w))*
                        c[k + (j<<1) + (i<<2)];
            }
        }
    }
    accum
}
