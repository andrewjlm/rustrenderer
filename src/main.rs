use std::io;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::slice;
use std::error::Error;
use std::io::{BufReader, BufRead};

// TODO: Probably some stuff with bits per pixel, I guess 24 for now (BGR, no alpha)
// Somewhat based on https://gist.github.com/jonvaldes/607fbc380f816d205afb
// TODO: Are we writing in the wrong endian order? Maybe use byteorder crate
#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

const BLACK: Color = Color(0, 0, 0);
const WHITE: Color = Color(255, 255, 255);
const RED: Color = Color(0, 0, 255);

struct Image {
    width: i32,
    height: i32,
    // TODO: Shouldn't this be an array?
    data: Vec<Color>,
}

struct Model {
    verts: Vec<Vec<f64>>,
    faces: Vec<Vec<usize>>,
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
    fn new(width: i32, height: i32) -> Self {
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

    fn write_tga_file(self: &Image, filename: &str) -> io::Result<()> {
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

impl Model {
    fn new(filename: &str) -> Self {
        let mut verts = Vec::new();
        let mut faces = Vec::new();

        let mut file = match File::open(&filename) {
            Err(why) => panic!("Couldn't open {}: {}", filename,
                               why.description()),
            Ok(file) => file,
        };

        let file = BufReader::new(file);

        for line in file.lines() {
            let line = line.unwrap();
            if !line.is_empty() {
                // Check the first character to figure out what we're looking at
                let mut split_line = line.split_whitespace();
                match split_line.next() {
                    Some("v") => {
                        // Parse vertices into a Vec of floats and add to the model struct
                        let vertices = split_line.filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
                        verts.push(vertices);
                    },
                    Some("f") => {
                        // We only care about the first number after the line (for now?)
                        let face = split_line.filter_map(|s| s.split('/').next().unwrap()
                                                         .parse::<usize>().ok()).collect::<Vec<_>>();
                        // Subtract one because they don't zero index :(
                        let face = face.iter().map(|f| (f - 1)).collect();
                        faces.push(face);
                    }
                    _ => {

                    }
                }
            }
        }

        println!("vertices: {}, faces: {}", verts.len(), faces.len());

        Model {
            verts: verts,
            faces: faces,
        }
    }
}

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: Color) {
    // We need to work in floats, then output in i32
    let x0 = x0 as f64;
    let x1 = x1 as f64;
    let y0 = y0 as f64;
    let y1 = y1 as f64;

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
                    true => {
                        y + 1.0
                    },
                    false => {
                        y - 1.0
                    }
                }
            },
            false => { y }
        };

        x = x + 1.0;
    }
}

fn main() {
    let (width, height) = (800, 800);
    let mut image = Image::new(width, height);
    let model = Model::new("obj/african_head.obj");
    // Iterate over the faces in the model and draw the triangles
    for face in model.faces {
        for idx in 0..3 {
            let ref v0 = model.verts[face[idx]];
            let ref v1 = model.verts[face[(idx + 1) % 3]];

            let x0 = ((v0[0] + 1.0) * (width as f64) / 2.0) as i32;
            let y0 = ((v0[1] + 1.0) * (height as f64) / 2.0) as i32;
            let x1 = ((v1[0] + 1.0) * (width as f64) / 2.0) as i32;
            let y1 = ((v1[1] + 1.0) * (height as f64) / 2.0) as i32;
            line(x0, y0, x1, y1, &mut image, WHITE);
        }
    }
    image.write_tga_file("output.tga");
}
