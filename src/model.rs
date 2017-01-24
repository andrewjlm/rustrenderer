use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

use image::*;
use geo::{Vec2, Vec3, cross_product};
use num::ToPrimitive;

pub struct Model {
    // TODO: Not sure if these should be public
    pub verts: Vec<Vec<f64>>,
    pub faces: Vec<Vec<usize>>,
    face_text: Vec<Vec<usize>>,
    pub text_coords: Vec<Vec<f64>>
}

// TODO: Implement function to draw the whole model
impl Model {
    pub fn new(filename: &str) -> Self {
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut face_text = Vec::new();
        let mut text_coords = Vec::new();

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
                        let mut face = Vec::new();
                        let mut texture = Vec::new();

                        for block in split_line {
                            let content = block.split('/').map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();
                            face.push(content[0].unwrap());
                            texture.push(content[1].unwrap());
                        }

                        // Subtract one because they don't zero index :(
                        let face = face.iter().map(|f| (f - 1)).collect();
                        let texture = texture.iter().map(|f| (f - 1)).collect();

                        faces.push(face);
                        face_text.push(texture);
                    },
                    Some("vt") => {
                        // Parse texture coordinates
                        let coords = split_line.filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
                        text_coords.push(coords);
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
            face_text: face_text,
            text_coords: text_coords,
        }
    }

    pub fn draw(self, mut image: &mut Image, light_dir: Vec3<i32>) {
        // Iterate over the faces in the model and draw the triangles
        for face in self.faces {
            let mut tri = Vec::new();
            let mut coords = Vec::new();

            let mut draw = false;

            for idx in 0..3 {
                let ref v = self.verts[face[idx]];
                tri.push(Vec3{x: ((v[0] + 1.0) * (image.width as f64) / 2.0) as i32,
                              y: ((v[1] + 1.0) * (image.height as f64) / 2.0) as i32,
                              z: v[2] as i32});
                coords.push(Vec3{x: v[0], y: v[1], z: v[2]});
            }

            // Calculate the surface normal of the triangle
            let v = coords[1] - coords[0];
            let w = coords[2] - coords[0];
            let norm = cross_product(v, w);
            let normalized = norm.normalize();

            let intensity = normalized * light_dir.to_f64();

            if intensity > 0.0 {
                // TODO: Probably a cleaner way to do this
                let shade = Color((intensity * 255.0).to_u8().unwrap(),
                                  (intensity * 255.0).to_u8().unwrap(),
                                  (intensity * 255.0).to_u8().unwrap());
                bb_triangle(tri[0], tri[1], tri[2], &mut image, shade);
            }
        }
    }
}
