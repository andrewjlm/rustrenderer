use std::fmt;
use std::ops::{Add, Sub, Mul};
use num::ToPrimitive;

pub trait VecNum: Add + Sub + Mul + Sized + ToPrimitive + Copy {}

// Apparently it's considered bad to do this but I don't know how to avoid...
impl VecNum for f64 {}
impl VecNum for i32 {}

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T: VecNum>  {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> where T: VecNum {
    pub fn new(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn to_f64(self) -> Vec2<f64> {
        Vec2 {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
        }
    }

    pub fn to_i32(self) -> Vec2<i32> {
        Vec2 {
            x: self.x.to_i32().unwrap(),
            y: self.y.to_i32().unwrap(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> where T: VecNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Vec2<T> where T: VecNum + Add<Output=T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: VecNum + Sub<Output=T> {
    type Output = Vec2<T>;

    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Scalar multiplication
impl<T> Mul<f64> for Vec2<T> where T: VecNum {
    type Output = Vec2<f64>;

    fn mul(self, rhs: f64) -> Vec2<f64> {
        Vec2 {
            x: self.x.to_f64().unwrap() * rhs,
            y: self.y.to_f64().unwrap() * rhs,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T: VecNum>{
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> where T: VecNum + Mul<T, Output=T> + Add<T, Output=T> {
    fn norm(self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).to_f64().unwrap().sqrt()
    }

    pub fn normalize(self) -> Vec3<f64> {
        let norm = self.norm();
        Vec3 {
            x: self.x.to_f64().unwrap() / norm,
            y: self.y.to_f64().unwrap() / norm,
            z: self.z.to_f64().unwrap() / norm,
        }
    }

    pub fn to_f64(self) -> Vec3<f64> {
        Vec3 {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
            z: self.z.to_f64().unwrap(),
        }
    }
}

impl<T> Sub for Vec3<T> where T: VecNum + Sub<Output=T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T> where T: VecNum + Add<T, Output=T> + Mul<T, Output=T> {
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl<T> Mul<f64> for Vec3<T> where T: VecNum {
    type Output = Vec3<f64>;

    fn mul(self, rhs: f64) -> Vec3<f64> {
        Vec3 {
            x: self.x.to_f64().unwrap() * rhs,
            y: self.y.to_f64().unwrap() * rhs,
            z: self.z.to_f64().unwrap() * rhs,
        }
    }
}

pub fn cross_product<T>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T>
    where T: VecNum + Mul<T, Output=T> + Sub<T, Output=T> {
    Vec3 {
        x: (a.y * b.z) - (a.z * b.y),
        y: (a.z * b.x) - (a.x * b.z),
        z: (a.x * b.y) - (a.y * b.x)
    }
}
