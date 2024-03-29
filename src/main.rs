use std::io::Write;
use std::time::Instant;
use std::{fs::File, io::Error};

use rand::{thread_rng, Rng};

use raytracing::camera::Camera;
use raytracing::hittable::{HitRecord, Hittable, HittableList};
use raytracing::material::{Dielectric, Lambertian, Material, Metal, Scatterable};
use raytracing::ray::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec3::{Color, Point3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: usize = 100;
    let max_depth = 15;

    println!("Image size: {}x{}", image_width, image_height);

    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let sphere1 = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(material_ground),
    );
    let sphere2 = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(material_center),
    );
    let sphere3 = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(material_left),
    );
    let sphere4 = Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(material_right),
    );
    let sphere5 = Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Material::Dielectric(material_left),
    );
    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);
    world.add(sphere5);

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(aspect_ratio, viewport_height, focal_length);

    // Generate pixels
    let mut pixels: Vec<Color> = vec![];

    let now = Instant::now();

    let mut rng = thread_rng();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rng.gen_range(0.0..=1.0)) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + rng.gen_range(0.0..=1.0)) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world.objects, max_depth);
            }
            let final_color = generate_color(pixel_color, samples_per_pixel);
            pixels.push(final_color);
        }
    }

    let time_elapsed = now.elapsed().as_secs();

    println!("Finished generating. Pixel count: {}", pixels.len());
    println!("Time elapsed: {} s", time_elapsed);

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

fn hit_world<'a>(world: &'a [Sphere], r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
    let mut closest = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest) {
            closest = hit.t;
            hit_record = Some(hit);
        }
    }

    hit_record
}

fn ray_color(r: &Ray, world: &[Sphere], depth: isize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit = hit_world(world, r, 0.001, f64::INFINITY);
    match hit {
        Some(record) => {
            let scattered = record.material.scatter(r, &record);
            match scattered {
                Some((albedo, scattered_ray)) => {
                    let target_color = ray_color(&scattered_ray, world, depth - 1);
                    Color::new(
                        albedo.x * target_color.x,
                        albedo.y * target_color.y,
                        albedo.z * target_color.z,
                    )
                }
                None => Color::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let t = 0.5 * (r.direction.unit_vector().y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn generate_color(pixel_color: Color, samples_per_pixel: usize) -> Color {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let new_r = 256.0 * clamp(r, 0.0, 0.999);
    let new_g = 256.0 * clamp(g, 0.0, 0.999);
    let new_b = 256.0 * clamp(b, 0.0, 0.999);

    Color::new(new_r, new_g, new_b)
}

fn write_to_file(width: u32, height: u32, pixels: &[Color]) -> Result<(), Error> {
    let mut file = File::create("output.ppm")?;

    let header = format!("P3\n{} {}\n255\n", width, height);

    write!(file, "{}", header)?;

    for pixel in pixels {
        let r = pixel.x as usize;
        let g = pixel.y as usize;
        let b = pixel.z as usize;
        file.write_fmt(format_args!("{} {} {}\n", r, g, b)).unwrap();
    }

    Ok(())
}
