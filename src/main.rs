mod image;
mod model;

use image::*;
use model::*;

fn main() {
    let (width, height) = (800, 800);
    let mut image = Image::new(width, height);
    // TODO: Command line argument for the object file
    let model = Model::new("obj/african_head.obj");
    model.draw(&mut image);
    image.write_tga_file("output.tga");
}
