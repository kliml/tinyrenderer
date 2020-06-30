use image::{ ImageBuffer, RgbImage, Pixel, Rgb };
use image::imageops::*;

fn main() {
    let white = Rgb::from_channels(255, 255, 255, 255);
    let red = Rgb::from_channels(255, 0, 0, 255);

    let (width, height) = (100, 100);
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    imgbuf.put_pixel(52, 41, red);
    imgbuf = flip_horizontal(&imgbuf);

    imgbuf.save("test.png").unwrap();
}
