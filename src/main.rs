mod ray;
mod vec3;

use std::io::{BufRead, BufReader, Write};
use std::{fs::File, io::Error};

use image::{Rgba, RgbaImage};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    println!("Image size: {}x{}", image_height, image_width);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Generate pixels
    let mut pixels: Vec<Color> = vec![];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color: Color = ray_color(r);
            pixels.push(color);
        }
    }

    // Render
    write_to_file(image_width, image_height, pixels).unwrap();
    //write_out_image(image_width, image_height, pixels);
}

fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.g + 1.0);
    let res = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    res
}

fn write_to_file(width: u32, height: u32, pixels: Vec<Color>) -> Result<(), Error> {
    let mut file = File::create("test_output.ppm")?;

    let header = format!("P3\n{} {}\n255\n", width, height).to_string();

    write!(file, "{}", header)?;

    for i in 0..pixels.len() {
        let r = (pixels[i].r * 255 as f64) as usize;
        let g = (pixels[i].g * 255 as f64) as usize;
        let b = (pixels[i].b * 255 as f64) as usize;
        write!(file, "{} {} {}\n", r, g, b).unwrap();
    }

    Ok(())
}

fn write_out_image(sizex: u32, sizey: u32, pixels: Vec<Color>) {
    let mut img = RgbaImage::new(sizex, sizey);

    for j in (0..sizey).rev() {
        for i in 0..sizex {
            let loc = (i + j * sizey) as usize;
            img.put_pixel(
                i,
                j,
                Rgba([
                    (pixels[loc].r * 255 as f64) as u8,
                    (pixels[loc].g * 255 as f64) as u8,
                    (pixels[loc].b * 255 as f64) as u8,
                    255,
                ]),
            );
        }
    }

    img.save("output.png").unwrap();
}
