use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

pub struct Model {
    // TODO: Not sure if these should be public
    pub verts: Vec<Vec<f64>>,
    pub faces: Vec<Vec<usize>>,
}

// TODO: Implement function to draw the whole model
impl Model {
    pub fn new(filename: &str) -> Self {
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