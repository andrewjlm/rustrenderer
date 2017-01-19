mod image;
mod model;

use image::*;
use model::*;

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
            image::line(x0, y0, x1, y1, &mut image, WHITE);
        }
    }
    image.write_tga_file("output.tga");
}
