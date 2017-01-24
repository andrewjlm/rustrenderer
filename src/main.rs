mod image;
mod model;
mod geo;
mod tga;

use image::*;
use model::*;
use geo::*;
use tga::*;

extern crate num;
extern crate byteorder;

fn main() {
    // let (width, height) = (200, 200);
    let (width, height) = (800, 800);
    // let mut image = Image::new(width, height);
    // TODO: Command line argument for the object file
    // let model = Model::new("obj/african_head/african_head.obj");
    let image = read_tga_file("obj/diablo3_pose/diablo3_pose_diffuse.tga");
    // match read_tga_file("obj/diablo3_pose/diablo3_pose_diffuse.tga") {
    // let image = match read_tga_file("obj/african_head/african_head_diffuse.tga") {
    //     Ok(Image) => Image,
    //     Err(e) => println!("Error: {:?}", e)
    // };
    // let image = read_tga_file("obj/african_head/african_head_diffuse.tga");
    image.unwrap().write_tga_file("test.tga");
    // let model = Model::new("obj/diablo3_pose/diablo3_pose.obj");
    // let light_dir = Vec3{x: 1, y: 0, z: 0};
    // model.draw(&mut image, light_dir);
    // image.write_tga_file("output.tga");
}
