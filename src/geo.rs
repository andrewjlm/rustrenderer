use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// TODO: Make it easy to convert this to float64

#[derive(Copy, Clone)]
pub struct Vec3<T>{
    pub x: T,
    pub y: T,
    pub z: T
}
