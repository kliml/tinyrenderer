use image::imageops::*;
use image::{ImageBuffer, Pixel, Rgb};
use num;
use std::mem;
use tobj;
use rand::Rng;

mod geometry;
use geometry::ToSimpleMesh;
use geometry::{ Vec3f, Triangle };

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
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let (v0, v1, v2) = (&mut v0.clone(), &mut v1.clone(), &mut v2.clone());

    if v0.y > v1.y { mem::swap(v0, v1) };
    if v0.y > v2.y { mem::swap(v0, v2) };
    if v1.y > v2.y { mem::swap(v1, v2) };

    let total_height = v2.y - v0.y;

    for y in (v0.y as i32)..(v1.y as i32) {
        let segment_height: i32 = v1.y as i32 - v0.y as i32 + 1;
        let alpha: f32 = (y - v0.y as i32) as f32 / total_height as f32;
        let beta: f32 = (y - v0.y as i32) as f32 / segment_height as f32;
        let mut Ax = v0.x + (v2.x - v0.x) * alpha;
        let mut Ay = v0.y + (v2.y - v0.y) * alpha;
        let mut Bx = v0.x + (v1.x - v0.x) * beta;
        let mut By = v0.y + (v1.x - v0.y) * beta;
        if Ax > Bx { mem::swap(&mut Ax, &mut Bx); mem::swap(&mut Ay, &mut By)};
        for x in (Ax as i32)..(Bx as i32) {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
    for y in (v1.y as i32)..(v2.y as i32) {
        let segment_height: i32 = v2.y as i32 - v1.y as i32 + 1;
        let alpha: f32 = (y - v0.y as i32) as f32 / total_height as f32;
        let beta: f32 = (y - v1.y as i32) as f32 / segment_height as f32;
        let mut Ax = v0.x + (v2.x - v0.x) * alpha;
        let mut Ay = v0.y + (v2.y - v0.y) * alpha;
        let mut Bx = v1.x + (v2.x - v1.x) * beta;
        let mut By = v1.y + (v2.x - v1.y) * beta;
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

    let (width, height) = (800, 800);
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height + 1);

    let obj = tobj::load_obj("obj/african_head.obj", true);

    let (models, materials) = obj.unwrap();

    println!("Uploading model: {}", models[0].name);
    let mesh = &models[0].mesh;

    let object = mesh.to_simple_mesh();
    for t in object.triangles {
        let mut screen_coords: [Vec3f; 3] = t.vertices;
        for i in 0..3 {

            let world_coords = t.vertices[i]; 
            screen_coords[i] = Vec3f{
                x: (world_coords.x + 1.) * width as f32 / 2.,
                y: (world_coords.y + 1.) * height as f32 / 2.,
                z: 0.0,
            };

            // println!("{} {}", screen_coords[0].x, screen_coords[0].y);
            
            // let vert1 = triangle.vertices[i];
            // let vert2 = triangle.vertices[(i + 1) % 3];

            // let x0 = ((vert1.x +1.)*width as f32 *0.5) as i32;
            // let y0 = ((vert1.y +1.)*height as f32 *0.5) as i32;
            // let x1 = ((vert2.x +1.)*width as f32 *0.5) as i32;
            // let y1 = ((vert2.y +1.)*height as f32 *0.5) as i32; 

            //line(x0, y0, x1, y1, &mut imgbuf, white);
        }

        let mut rng = rand::thread_rng();
        let color = Rgb::from_channels(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 255);
        triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut imgbuf, color);
    }

    imgbuf = flip_vertical(&imgbuf);

    imgbuf.save("res/test.png").unwrap();
}
