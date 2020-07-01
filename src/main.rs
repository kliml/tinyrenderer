use image::{ ImageBuffer, Pixel, Rgb };
use image::imageops::*;
use num;
use std::mem;
use tobj;

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>) {
    let (mut x0, mut x1, mut y0, mut y1) = (x0, x1, y0, y1);
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
            y += if y1 > y0 {1} else {-1};
            error -= dx * 2; 
        }
    }
}

fn main() {
    let white = Rgb::from_channels(255, 255, 255, 255);
    let red = Rgb::from_channels(255, 0, 0, 255);

    let (width, height) = (800, 800);
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width + 1, height + 1);

    let obj = tobj::load_obj("obj/african_head.obj", true);
    
    let (models, materials) = obj.unwrap();

    println!("Uploading model: {}", models[0].name);
    let mesh = &models[0].mesh;
    
    let mut faces = vec![];
    let mut next_face = 0;
    for f in 0..mesh.num_face_indices.len() {
        let end = next_face + mesh.num_face_indices[f] as usize;
        let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        faces.push(face_indices);
        next_face = end;
    }

    let mut vertices = vec![];
    for v in (0..mesh.positions.len()).step_by(3) {
        let vertice: Vec<_> = mesh.positions[v..v+3].iter().collect();
        vertices.push(vertice);
    }

    for face in &faces {
        for i in 0..3 {
            let vert1 = &vertices[*face[i] as usize];
            let vert2 = &vertices[*face[(i+1)%3] as usize];

            let x0 = ((vert1[0]+1.)*width as f32 *0.5) as i32;
            let y0 = ((vert1[1]+1.)*height as f32 *0.5) as i32;
            let x1 = ((vert2[0]+1.)*width as f32 *0.5) as i32;
            let y1 = ((vert2[1]+1.)*height as f32 *0.5) as i32;

            line(x0, y0, x1, y1, &mut imgbuf, white);
        }
    }

    imgbuf = flip_vertical(&imgbuf);

    imgbuf.save("res/head.png").unwrap();
}
