use std::fmt;
use std::ops::{Add, Sub, Mul};
use num::ToPrimitive;
use image::{Image, Color};

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
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Vec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2]
        }
    }

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

    pub fn to_i32(self) -> Vec3<i32> {
        Vec3 {
            x: self.x.to_i32().unwrap(),
            y: self.y.to_i32().unwrap(),
            z: self.z.to_i32().unwrap(),
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

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub coords: Vec3<f64>,
    // Should this be an int?
    texture: Option<Vec2<f64>>,
    pub screen_coords: Vec3<f64>,
    screen_texture: Option<Vec2<f64>>,
}

impl Vertex {
    pub fn new(coords: Vec3<f64>) -> Self {
        Vertex {
            coords: coords,
            texture: None,
            screen_coords: coords,
            screen_texture: None
        }
    }

    pub fn from_vec(coords: Vec<f64>) -> Self {
        let coords = Vec3::from_vec(coords);
        Vertex {
            coords: coords,
            texture: None,
            screen_coords: coords,
            screen_texture: None
        }
    }

    pub fn set_texture(&mut self, coords: Vec<f64>) {
        self.texture = Some(Vec2 {
            x: coords[0],
            y: coords[1]
        });
    }

    pub fn scale_to_image(self, width: i32, height: i32) -> Vertex {
        let new_coords = Vec3{x: (self.coords.x + 1.0) * (width as f64) / 2.0,
                              y: (self.coords.y + 1.0) * (height as f64) / 2.0,
                              z: self.coords.z};
        Vertex {
            coords: self.coords,
            texture: self.texture,
            screen_coords: new_coords,
            screen_texture: None,
        }
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub vertices: Vec<Vertex>,
}

impl Triangle {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Triangle {
            vertices: vertices
        }
    }

    pub fn scale_to_image(self, width: i32, height: i32) -> Triangle {
        Triangle {
            vertices: self.vertices.iter().map(|v| v.scale_to_image(width, height)).collect()
        }
    }

    pub fn surface_normal(&self) -> Vec3<f64> {
        let v = self.vertices[1].coords - self.vertices[0].coords;
        let w = self.vertices[2].coords - self.vertices[0].coords;

        let norm = cross_product(v, w);
        norm.normalize()
    }

    pub fn draw(&self, mut image: &mut Image, color: Color) {
        let bbox: Vec<Vec2<i32>> = self.find_bounding_box();
        let bbox = self.clip_bounding_box(bbox, &image);
        // Line(bbox[0], bbox[1], &mut image, RED);
        // line(bbox[1], bbox[2], &mut image, RED);
        // line(bbox[2], bbox[3], &mut image, RED);
        // line(bbox[3], bbox[0], &mut image, RED);

        // Compute edge function for all 3 points - we'll use this to scale for zbuffer
        let area = barycentric(&self.vertices[0].screen_coords.to_i32(),
                               &self.vertices[1].screen_coords.to_i32(),
                               &self.vertices[2].screen_coords.to_i32());

        // Iterate over the pixels in the bounding box
        if area != 0 {
            for x in bbox[0].x..bbox[3].x {
                for y in bbox[0].y..bbox[2].y {
                    let p = Vec3{x: x, y: y, z: 0};
                    let bc1 = barycentric(&self.vertices[0].screen_coords.to_i32(),
                                          &self.vertices[1].screen_coords.to_i32(),
                                          &p);
                    let bc2 = barycentric(&self.vertices[1].screen_coords.to_i32(),
                                          &self.vertices[2].screen_coords.to_i32(),
                                          &p);
                    let bc3 = barycentric(&self.vertices[2].screen_coords.to_i32(),
                                          &self.vertices[0].screen_coords.to_i32(),
                                          &p);

                    // If any of the barycentric coordinates are negative, don't draw
                    if bc1 >= 0 && bc2 >= 0 && bc3 >= 0 {
                        // Scale the coordinates
                        let bc1 = bc1 / area;
                        let bc2 = bc2 / area;
                        let bc3 = bc3 / area;

                        // Compute the depth
                        let z = 1.0 / ((((self.vertices[0].screen_coords.z as i32 * bc1) as f64) +
                                        ((self.vertices[1].screen_coords.z as i32 * bc2) as f64) +
                                        ((self.vertices[2].screen_coords.z as i32 * bc3) as f64)));

                        if z > image.get_depth(x, y) {
                            image.set_depth(x, y, z);
                            image.set_pixel(x, y, color);
                        }
                    }
                }
            }
        }
    }

    pub fn find_bounding_box(&self) -> Vec<Vec2<i32>> {
        // Find coordinates of the corners of the bounding box
        let mut xs: Vec<i32> = self.vertices.iter().map(|v| v.screen_coords.x as i32).collect();
        let mut ys: Vec<i32> = self.vertices.iter().map(|v| v.screen_coords.y as i32).collect();
        xs.sort_by(|a, b| a.cmp(b));
        ys.sort_by(|a, b| a.cmp(b));

        let (min_x, max_x) = (xs[0], xs[2]);
        let (min_y, max_y) = (ys[0], ys[2]);

        vec![Vec2{x: min_x, y: min_y},
             Vec2{x: min_x, y: max_y},
             Vec2{x: max_x, y: max_y},
             Vec2{x: max_x, y: min_y}]
    }

    pub fn clip_bounding_box(&self, bbox: Vec<Vec2<i32>>, image: &Image) -> Vec<Vec2<i32>> {
        let mut result = Vec::with_capacity(4);

        for i in bbox {
            let clipped_bounds = Vec2{x: clip(i.x, 0, image.width),
                                      y: clip(i.y, 0, image.height)};
            result.push(clipped_bounds);
        }

        result
    }
}

pub fn barycentric(t0: &Vec3<i32>, t1: &Vec3<i32>, p: &Vec3<i32>) -> i32 {
    // Compute edge function
    (t1.x - t0.x) * (p.y - t0.y) - (t1.y - t0.y) * (p.x - t0.x)
}

fn clip(x: i32, min: i32, max: i32) -> i32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
