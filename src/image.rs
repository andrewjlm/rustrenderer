use std::io;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::slice;
use geo::Vec2;
use std::cmp;

// TODO: Probably some stuff with bits per pixel, I guess 24 for now (BGR, no alpha)
// Somewhat based on https://gist.github.com/jonvaldes/607fbc380f816d205afb
// TODO: Are we writing in the wrong endian order? Maybe use byteorder crate
#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

pub const BLACK: Color = Color(0, 0, 0);
pub const WHITE: Color = Color(255, 255, 255);
pub const RED: Color = Color(0, 0, 255);
pub const GREEN: Color = Color(0, 255, 0);
pub const BLUE: Color = Color(255, 0, 0);

pub struct Image {
    pub width: i32,
    pub height: i32,
    data: Vec<Color>,
}

// Given a reference to a T, return a reference to that slice
unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    // Convert the pointer into a u8
    let data_ptr: *const u8 = mem::transmute(s);
    // Return the slice to the end of s
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

// Same thing but for a reference to a slice
unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        // Initialize with black background
        let v = vec![BLACK; (width * height) as usize];

        Image {
            width: width,
            height: height,
            data: v,
        }
    }

    fn set_pixel(self: &mut Image, x: i32, y: i32, c: Color) {
        if !(x < 0 || y < 0 || x >= self.width || y >= self.width) {
            // The index in the vector is the width times y plus x
            self.data[((y * self.width) + x) as usize] = c;
        }
    }

    pub fn write_tga_file(self: &Image, filename: &str) -> io::Result<()> {
        // Do what C does, also strip any padding and align the type to a byte
        #[repr(C, packed)]
        #[derive(Default)]
        struct Header {
            id_length: u8,
            color_map_type: u8,
            data_type_code: u8,
            color_map_origin: u16,
            color_map_length: u16,
            color_map_depth: u8,
            x_origin: u16,
            y_origin: u16,
            width: u16,
            height: u16,
            bits_per_pixel: u8,
            image_descriptor: u8,
        }

        let h = Header {
            data_type_code: 2,
            width: self.width as u16,
            height: self.height as u16,
            bits_per_pixel: 24,
            ..Header::default()
        };

        let mut f = try!(File::create(filename));
        // Write the header struct and data into the file
        unsafe {
            try!(f.write_all(struct_to_u8_slice(&h)));
            try!(f.write_all(slice_to_u8_slice(&self.data[..])));
        }
        Ok(())
    }

    fn flip_vertically(&mut self) {
        self.data.reverse();
    }
}

pub fn line(point1: Vec2<i32>, point2: Vec2<i32>, image: &mut Image, color: Color) {
    // We need to work in floats, then output in i32
    let x0 = point1.x as f64;
    let x1 = point2.x as f64;
    let y0 = point1.y as f64;
    let y1 = point2.y as f64;

    // If the line is steep, we transpose the coordinates
    let steep = (x0 -x1).abs() < (y0 - y1).abs();
    let (x0, y0, x1, y1) = match steep {
        true => { (y0, x0, y1, x1) },
        false => { (x0, y0, x1, y1) }
    };

    // If we're drawing left to right, swap again
    let (x0, y0, x1, y1) = match x0 > x1 {
        true => { (x1, y1, x0, y0) },
        false => { (x0, y0, x1, y1) }
    };

    // Use change in x and y to keep track of error in pixels
    let dx = x1 - x0;
    let dy = y1 - y0;

    // Calculate pixel error - double the total vertical change
    let derror = (dy * 2.0).abs();
    let mut error = 0.0;

    let mut x = x0;
    let mut y = y0;
    while x <= x1 {
        match steep {
            true => {
                image.set_pixel(y as i32, x as i32, color);
            },
            false => {
                image.set_pixel(x as i32, y as i32, color);
            }
        }

        error += derror;

        // If our error is greater than the horizontal distance, increment y and reset
        y = match error > dx {
            true => {
                error -= dx * 2.0;
                match y1 > y0 {
                    true => { y + 1.0 },
                    false => { y - 1.0 }
                }
            },
            false => { y }
        };

        x = x + 1.0;
    }
}

// TODO: Should create structs for 2 and 3d vectors
// TODO: Are we using references and mutability in a consistent way?
pub fn triangle(t0: Vec2<i32>, t1: Vec2<i32>, t2: Vec2<i32>, mut image: &mut Image, color: Color) {
    line(t0, t1, &mut image, color);
    line(t1, t2, &mut image, color);
    line(t2, t0, &mut image, color);
}

pub fn bb_triangle(t0: Vec2<i32>, t1: Vec2<i32>, t2: Vec2<i32>, mut image: &mut Image, color: Color) {
    // TODO: Should return a tuple, maybe?
    let bbox: Vec<Vec2<i32>> = find_bounding_box(t0, t1, t2);
    let bbox = clip_bounding_box(bbox, &image);

    // Iterate over pixels in bounding box
    for x in bbox[0].x..bbox[3].x {
        for y in bbox[0].y..bbox[2].y {
            let p = Vec2{x: x, y: y};
            let bc1 = barycentric(t0, t1, p);
            let bc2 = barycentric(t1, t2, p);
            let bc3 = barycentric(t2, t0, p);

            // If any of the barycentric coordinates are negative, don't draw
            if bc1 >= 0 && bc2 >= 0 && bc3 >= 0 {
                image.set_pixel(x, y, color);
            }
        }
    }
}

fn find_bounding_box(t0: Vec2<i32>, t1: Vec2<i32>, t2: Vec2<i32>) -> Vec<Vec2<i32>> {
    // Find coordinates of the corners of the bounding box
    let mut xs = vec![t0.x, t1.x, t2.x];
    let mut ys = vec![t0.y, t1.y, t2.y];
    xs.sort_by(|a, b| a.cmp(&b));
    ys.sort_by(|a, b| a.cmp(&b));

    let (min_x, max_x) = (xs[0], xs[2]);
    let (min_y, max_y) = (ys[0], ys[2]);

    vec![Vec2{x: min_x, y: min_y},
         Vec2{x: min_x, y: max_y},
         Vec2{x: max_x, y: max_y},
         Vec2{x: max_x, y: min_y}]
}

fn clip_bounding_box(bbox: Vec<Vec2<i32>>, image: &Image) -> Vec<Vec2<i32>> {
    let mut result = Vec::new();

    for i in bbox  {
        result.push(Vec2{x: clip(i.x, 0, image.width),
                         y: clip(i.y, 0, image.height)});
    }

    result
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

fn barycentric(t0: Vec2<i32>, t1: Vec2<i32>, p: Vec2<i32>) -> i32 {
    // Compute edge function
    (p.x - t0.x) * (t1.y - t0.y) - (p.y - t0.y) * (t1.x - t0.x)
}

pub fn filled_triangle(t0: Vec2<i32>, t1: Vec2<i32>, t2: Vec2<i32>, mut image: &mut Image, color: Color) {
    // TODO: Better way to convert everything to f64
    let mut v = vec![t0, t1, t2];
    v.sort_by(|a, b| a.y.cmp(&b.y));

    // Check for flat-top/bottom triangle, which are easy
    if v[1].y == v[2].y {
        flat_top_triangle(v, &mut image, color);
    } else if v[0].y == v[1].y {
        flat_bottom_triangle(v, &mut image, color);
    } else {
        // Find the middle of the triangle and divide/conquer
        // We already know the y - to find x, we add the ratio of
        // height differences times the width distance (intercept theorem)
        let height_ratio = ((v[1].y - v[0].y) as f64) / ((v[2].y - v[0].y) as f64);
        let width = (v[2].x - v[0].x) as f64;
        let new_x = ((v[0].x as f64) + height_ratio * width) as i32;
        let new_v = Vec2{x: new_x, y: v[1].y};
        flat_top_triangle(vec![v[0], v[1], new_v], &mut image, color);
        flat_bottom_triangle(vec![v[1], new_v, v[2]], &mut image, color);
    }
}

fn flat_top_triangle(v: Vec<Vec2<i32>>, mut image: &mut Image, color: Color) {
    // Find the slope in each y direction
    let slope1 = ((v[1].x - v[0].x) as f64) / ((v[1].y - v[0].y) as f64);
    let slope2 = ((v[2].x - v[0].x) as f64) / ((v[2].y - v[0].y) as f64);

    // Keep track of current X bounds
    let mut x1 = v[0].x as f64;
    let mut x2 = v[0].x as f64;

    for y in v[0].y..v[1].y {
        // Draw line across the triangle
        line(Vec2{x: x1 as i32, y: y}, Vec2{x: x2 as i32, y: y}, &mut image, color);

        x1 += slope1;
        x2 += slope2;
    }
}

fn flat_bottom_triangle(v: Vec<Vec2<i32>>, mut image: &mut Image, color: Color) {
    let slope1 = ((v[2].x - v[0].x) as f64) / ((v[2].y - v[0].y) as f64);
    let slope2 = ((v[2].x - v[1].x) as f64) / ((v[2].y - v[1].y) as f64);

    let mut x1 = v[2].x as f64;
    let mut x2 = v[2].x as f64;

    for y in (v[1].y..v[2].y).rev() {
        line(Vec2{x: x1 as i32, y: y}, Vec2{x: x2 as i32, y: y}, &mut image, color);

        x1 -= slope1;
        x2 -= slope2;
    }
}
