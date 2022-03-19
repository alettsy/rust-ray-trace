mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use rand::{thread_rng, Rng};
use std::f64::INFINITY;
use std::io::Write;
use std::{fs::File, io::Error};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: usize = 100;

    println!("Image size: {}x{}", image_height, image_width);

    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&sphere1);
    world.add(&sphere2);

    // Camera
    let camera = Camera::new();

    // Generate pixels
    let mut pixels: Vec<Color> = vec![];

    let mut rng = thread_rng();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rng.gen_range(0.0..=1.0)) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + rng.gen_range(0.0..=1.0)) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            let final_color = generate_color(pixel_color, samples_per_pixel);
            pixels.push(final_color);
        }
    }

    // Render
    write_to_file(image_width, image_height, &pixels).unwrap();
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new_empty();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn generate_color(pixel_color: Color, samples_per_pixel: usize) -> Color {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let new_r = 256.0 * clamp(r, 0.0, 0.999);
    let new_g = 256.0 * clamp(g, 0.0, 0.999);
    let new_b = 256.0 * clamp(b, 0.0, 0.999);

    Color::new(new_r, new_g, new_b)
}

fn write_to_file(width: u32, height: u32, pixels: &Vec<Color>) -> Result<(), Error> {
    let mut file = File::create("output.ppm")?;

    let header = format!("P3\n{} {}\n255\n", width, height).to_string();

    write!(file, "{}", header)?;

    for i in 0..pixels.len() {
        let r = pixels[i].x as usize;
        let g = pixels[i].y as usize;
        let b = pixels[i].z as usize;
        write!(file, "{} {} {}\n", r, g, b).unwrap();
    }

    Ok(())
}

// fn _write_out_image(sizex: u32, sizey: u32, pixels: Vec<Color>) {
//     let mut img = RgbaImage::new(sizex, sizey);

//     for j in (0..sizey).rev() {
//         for i in 0..sizex {
//             let loc = (i + j * sizey) as usize;
//             img.put_pixel(
//                 i,
//                 j,
//                 Rgba([
//                     (pixels[loc].x * 255 as f64) as u8,
//                     (pixels[loc].y * 255 as f64) as u8,
//                     (pixels[loc].z * 255 as f64) as u8,
//                     255,
//                 ]),
//             );
//         }
//     }

//     img.save("output.png").unwrap();
// }

// fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * PI / 180.0
// }

// fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
//     let oc: Vec3 = r.origin - center;
//     let a = r.direction.length_squared();
//     let half_b = oc.dot(r.direction);
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / a
//     }
// }
