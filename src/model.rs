use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

use image::*;
use geo::{Vertex, Vec3, Triangle};
use num::ToPrimitive;
use tga::read_tga_file;

fn find_relative_file(origin: &str, relation: &str) -> PathBuf {
    let mut path = PathBuf::from(origin);
    let filename = path.file_stem().unwrap().to_os_string().into_string();

    let new_filename = match filename {
        Ok(s) => {
            s + relation
        },
        Err(e) => {
            panic!("Failed to produce relative path: {:?}", e)
        }
    };

    path.set_file_name(new_filename);
    path
}

pub struct Model {
    // TODO: Not sure if these should be public
    pub faces: Vec<Triangle>,
    texture: Option<Image>,
}

impl Model {
    pub fn new(filename: &str) -> Self {
        let mut verts: Vec<Vertex> = Vec::new();
        let mut face_idxs: Vec<Vec<usize>> = Vec::new();
        let mut face_text: Vec<Vec<usize>> = Vec::new();
        let mut text_coords = Vec::new();

        // We'll assume that the texture is in a file in the same directory
        let texture_file = find_relative_file(filename, "_diffuse.tga");

        let model_file = match File::open(&filename) {
            Err(why) => panic!("Couldn't open {}: {}", filename,
                               why.description()),
            Ok(model_file) => model_file,
        };

        let model_file = BufReader::new(model_file);
        let texture = read_tga_file(texture_file.to_str().unwrap());

        for line in model_file.lines() {
            let line = line.unwrap();
            if !line.is_empty() {
                // Check the first character to figure out what we're looking at
                let mut split_line = line.split_whitespace();
                match split_line.next() {
                    Some("v") => {
                        // Parse vertices into a Vertex and add to the model struct
                        let vertices = split_line.filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
                        let vertex = Vertex::from_vec(vertices);
                        verts.push(vertex);
                    },
                    Some("f") => {
                        // The first number has the idxs of the corners for the face
                        // The second number has the coords of the texture
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

                        face_idxs.push(face);
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

        println!("vertices: {}, faces: {}", verts.len(), face_idxs.len());

        let mut faces: Vec<Triangle> = Vec::new();

        for (face_idx, text_idx) in face_idxs.iter().zip(face_text.iter()) {
            let mut vertices: Vec<Vertex> = Vec::with_capacity(3);

            for (idx, f) in face_idx.iter().enumerate() {
                let mut vert = verts[*f];
                vert.set_texture(vec![text_coords[text_idx[idx]][0],
                                 text_coords[text_idx[idx]][1]]);
                vertices.push(vert);
            }

            let tri = Triangle::new(vertices);
            faces.push(tri);
        }

        Model {
            faces: faces,
            texture: texture.ok()
        }
    }

    pub fn draw(&self, mut image: &mut Image, light_dir: Vec3<i32>) {
        // Iterate over the faces in the model and draw the triangles
        for face in self.faces.iter() {

            // Scale the triangle to the screen size
            let tri = &face.scale_to_image(image.width, image.height);

            // Calculate the surface normal
            let norm = tri.surface_normal();
            let intensity = norm * light_dir.to_f64();

            if intensity > 0.0 {
                // let shade = Color((intensity * 255.0).to_u8().unwrap(),
                //                   (intensity * 255.0).to_u8().unwrap(),
                //                   (intensity * 255.0).to_u8().unwrap());
                let shade = Color((intensity * 255.0).to_u8().unwrap(),
                                  (intensity * 255.0).to_u8().unwrap(),
                                  (intensity * 255.0).to_u8().unwrap());
                let texture = &self.texture;
                tri.draw(&mut image, shade, texture.as_ref().unwrap());
                // bb_triangle(tri.vertices[0].screen_coords.to_i32(),
                //             tri.vertices[1].screen_coords.to_i32(),
                //             tri.vertices[2].screen_coords.to_i32(),
                //             &mut image,
                //             shade,
                //             tri);
            }
        }
    }
}
