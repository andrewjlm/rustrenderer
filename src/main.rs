mod image;
mod model;
mod geo;
mod tga;

use image::*;
use model::*;
use geo::*;

extern crate num;
extern crate byteorder;

fn main() {
    let (width, height) = (800, 800);
    let mut image = Image::new(width, height);
    // TODO: Command line argument for the object file
    let model = Model::new("obj/african_head/african_head.obj");
    // let model = Model::new("obj/diablo3_pose/diablo3_pose.obj");
    let light_dir = Vec3{x: 0, y: 0, z: 1};
    model.draw(&mut image, light_dir);
    image.write_tga_file("output.tga");
}
