mod image;
mod model;
mod geo;

use image::*;
use model::*;
use geo::*;

fn main() {
    // let (width, height) = (200, 200);
    let (width, height) = (800, 800);
    let mut image = Image::new(width, height);
    // TODO: Command line argument for the object file
    // let model = Model::new("obj/african_head.obj");
    let t0: Vec<Vec2<i32>> = vec![Vec2{x: 10, y: 70},
                                  Vec2{x: 50, y: 160},
                                  Vec2{x: 70, y: 80}];
    let t1: Vec<Vec2<i32>> = vec![Vec2{x: 180, y: 50},
                                  Vec2{x: 150, y: 1},
                                  Vec2{x: 70, y: 180}];
    let t2: Vec<Vec2<i32>> = vec![Vec2{x: 180, y: 150},
                                  Vec2{x: 120, y: 160},
                                  Vec2{x: 130, y: 180}];
    // triangle(t0[0], t0[1], t0[2], &mut image, RED);
    // triangle(t1[0], t1[1], t1[2], &mut image, WHITE);
    // triangle(t2[0], t2[1], t2[2], &mut image, GREEN);
    bb_triangle(t0[0], t0[1], t0[2], &mut image, RED);
    bb_triangle(t1[0], t1[1], t1[2], &mut image, WHITE);
    bb_triangle(t2[0], t2[1], t2[2], &mut image, GREEN);
    image.write_tga_file("output.tga");
}
