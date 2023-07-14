pub use super::float::*;

pub struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self::new(float_zero(), float_zero(), float_zero())
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
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<&Vec3> for Float {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Mul<Float> for &Vec3 {
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
