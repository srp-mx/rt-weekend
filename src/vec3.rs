pub use super::float::*;
pub use super::rng_float::*;

pub struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn random(rng: &mut RngGen) -> Self {
        Self::new(rng.get(), rng.get(), rng.get())
    }

    pub fn random_range(rng: &mut RngGen, min: Float, max: Float) -> Self {
        Self::new(rng.range(min, max), rng.range(min, max), rng.range(min, max))
    }

    pub fn random_sphere(rng: &mut RngGen) -> Self {
        loop {
            let p = Self::random_range(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn new(x: Float, y: Float, z:Float) -> Self {
        Self { x, y, z }
    }

    pub fn copy(&self) -> Self {
        Self::new(self.x, self.y, self.z)
    }

    pub fn x(&self) -> Float {
        self.x
    }

    pub fn y(&self) -> Float {
        self.y
    }

    pub fn z(&self) -> Float {
        self.z
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Float {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn dot(u: &Self, v: &Self) -> Float {
        u.x*v.x + u.y*v.y + u.z*v.z
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y*v.z - u.z*v.y,
            y: u.z*v.x - u.x*v.z,
            z: u.x*v.y - u.y*v.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn lerp(orig:&Self, dest:&Self, t:Float) -> Vec3 {
        (1.0 - t) * orig + t * dest
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl std::ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<&Vec3> for Float {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl std::ops::Mul<Float> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Float) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Float) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Float> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Float) -> Self::Output {
        rhs.recip() * self
    }
}

impl std::ops::Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Float) -> Self::Output {
        rhs.recip() * self
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        *self *= rhs.recip();
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
