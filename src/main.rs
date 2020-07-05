use image::imageops::*;
use image::{ImageBuffer, Pixel, Rgb};
use num;
use std::mem;
use tobj;
use rand::Rng;

mod geometry;
use geometry::ToSimpleMesh;
use geometry::{ Vec3f, Vec3i, Triangle };

fn line(
    v0: &Vec3f,
    v1: &Vec3f,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let (mut x0, mut x1, mut y0, mut y1) = (v0.x as i32, v1.x as i32, v0.y as i32, v1.y as i32);
    let mut steep = false;
    if num::abs(x0 - x1) < num::abs(y0 - y1) {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror = num::abs(dy) * 2;
    let mut error = 0;
    let mut y = y0;
    for x in x0..x1 {
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}


fn triangle(
    v0: Vec3i,
    v1: Vec3i,
    v2: Vec3i,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let (v0, v1, v2) = (&mut v0.clone(), &mut v1.clone(), &mut v2.clone());

    if v0.y > v1.y { mem::swap(v0, v1) };
    if v0.y > v2.y { mem::swap(v0, v2) };
    if v1.y > v2.y { mem::swap(v1, v2) };

    let total_height = v2.y - v0.y;

    for y in v0.y..v1.y {
        let segment_height: i32 = v1.y - v0.y + 1;
        let alpha: f32 = (y - v0.y) as f32 / total_height as f32;
        let beta: f32 = (y - v0.y) as f32 / segment_height as f32;
        let mut Ax = v0.x + ((v2.x - v0.x) as f32 * alpha) as i32;
        let mut Ay = v0.y + ((v2.y - v0.y) as f32 * alpha) as i32;
        let mut Bx = v0.x + ((v1.x - v0.x) as f32 * beta) as i32;
        let mut By = v0.y + ((v1.x - v0.y) as f32 * beta) as i32;
        if Ax > Bx { mem::swap(&mut Ax, &mut Bx); mem::swap(&mut Ay, &mut By)};
        for x in (Ax as i32)..(Bx as i32) {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
    for y in v1.y..v2.y  {
        let segment_height: i32 = v2.y - v1.y + 1;
        let alpha: f32 = (y - v0.y) as f32 / total_height as f32;
        let beta: f32 = (y - v1.y) as f32 / segment_height as f32;
        let mut Ax = v0.x + ((v2.x - v0.x) as f32 * alpha) as i32;
        let mut Ay = v0.y + ((v2.y - v0.y) as f32 * alpha) as i32;
        let mut Bx = v1.x + ((v2.x - v1.x) as f32 * beta) as i32;
        let mut By = v1.y + ((v2.x - v1.y) as f32 * beta) as i32;
        if Ax > Bx { mem::swap(&mut Ax, &mut Bx); mem::swap(&mut Ay, &mut By)};
        for x in (Ax as i32)..(Bx as i32) {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
}

fn main() {
    let white = Rgb::from_channels(255, 255, 255, 255);
    let red = Rgb::from_channels(255, 0, 0, 255);
    let green = Rgb::from_channels(0, 255, 0, 255);

    let light_dir = Vec3f{ x: 0.0, y: 0.0, z: -1.0 };

    let (width, height) = (800, 800);
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let obj = tobj::load_obj("obj/african_head.obj", true);

    let (models, materials) = obj.unwrap();

    println!("Uploading model: {}", models[0].name);
    let mesh = &models[0].mesh;

    let object = mesh.to_simple_mesh();
    for t in object.triangles {
        let mut screen_coords: [Vec3i; 3] = [Vec3i{x:0, y:0,z:0};3];
        let mut world_coords:  [Vec3f; 3] = [Vec3f{x:0.0, y:0.0,z:0.0};3];
        for i in 0..3 {

            let v = t.vertices[i]; 
            screen_coords[i] = Vec3i{
                x: ((v.x + 1.) * width as f32 / 2.) as i32,
                y: ((v.y + 1.) * height as f32 / 2.) as i32,
                z: 0,
            };
            world_coords[i] = v;
        }
        let mut n = (world_coords[2]-world_coords[0])^(world_coords[1] - world_coords[0]);
        let normalization_multiplier = 1.0 / n.norm(); 
        n = n * normalization_multiplier as f32;
        let intensity = n.scalar_mul(&light_dir);
        eprintln!("{}", intensity);
        if intensity > 0.0 {
            let color = (intensity * 255.0) as u8;
            let color = Rgb::from_channels(color, color, color, 255);
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut imgbuf, color);
        }

        // Colored head
        // let mut rng = rand::thread_rng();
        // let color = Rgb::from_channels(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 255);
        // triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut imgbuf, color);
    }

    imgbuf = flip_vertical(&imgbuf);

    imgbuf.save("res/head2.png").unwrap();
}
