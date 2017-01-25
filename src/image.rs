use std::io;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::slice;
use geo::{Vec2};
use std::f64;

// TODO: Probably some stuff with bits per pixel, I guess 24 for now (BGR, no alpha)
// Somewhat based on https://gist.github.com/jonvaldes/607fbc380f816d205afb
#[derive(Clone, Copy, Debug)]
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
    zbuffer: Vec<f64>,
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
        // The z-buffer is the same size as the image and initially filled with -Inf (basically)
        let z = vec![f64::MIN; (width * height) as usize];

        Image {
            width: width,
            height: height,
            data: v,
            // TODO: The zbuffer should probably be in some other object - a scene?
            zbuffer: z,
        }
    }

    pub fn set_pixel(self: &mut Image, x: i32, y: i32, c: Color) {
        if !(x < 0 || y < 0 || x >= self.width || y >= self.width) {
            // The index in the vector is the width times y plus x
            self.data[((y * self.width) + x) as usize] = c;
        }
    }

    pub fn set_data_buffer(self: &mut Image, data: Vec<Color>) {
        self.data = data;
    }

    // TODO: Should the zbuffer be floats?
    pub fn set_depth(self: &mut Image, x: i32, y: i32, d: f64) {
        if !(x < 0 || y < 0 || x >= self.width || y >= self.width) {
            self.zbuffer[((y * self.width) + x) as usize] = d;
        }
    }

    pub fn get_depth(self: &Image, x: i32, y: i32) -> f64 {
        self.zbuffer[((y * self.width) + x) as usize]
    }

    pub fn write_tga_file(self: &Image, filename: &str) -> io::Result<()> {
        // Do what C does, also strip any padding and align the type to a byte
        // TODO: Move this to TGA
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
    let p1 = point1.to_f64();
    let p2 = point2.to_f64();
    let x0 = p1.x;
    let x1 = p2.x;
    let y0 = p1.y;
    let y1 = p2.y;

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
