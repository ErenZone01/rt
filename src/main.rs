mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod cylindre;
mod cube;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{ scatter, Material };
use ray::Ray;
use sphere::Sphere;
use cylindre::Cylinder;
use cube::Cube;
use vec3::{ Vec3, Point3 }; // Import Point3 alias

use indicatif::{ ProgressBar, ProgressStyle };
use rand::prelude::*;
use rayon::prelude::*;
use std::time;

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        let intensity = 1.5; // Augmenter cette valeur pour plus de lumière
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) * intensity + Vec3::new(0.5, 0.7, 1.0) * t * intensity
    }
}

use std::env;
fn main() {
    let width = 400;
    let height = 200;
    let samples = 100;
    let max_value = 255;
    let args: Vec<String> = env::args().collect();
    let mut actif = false;
    if args.len() >= 2 {
        if !args[1].is_empty() {
            let mut list: Vec<Box<dyn Hittable>> = Vec::new();
            // Ajoute une grande sphère au sol
            list.push(
                Box::new(
                    Sphere::sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian {
                        albedo: Vec3::new(0.8, 0.3, 0.5),
                    })
                )
            );

            let aspect_ratio = (width as f32) / (height as f32);
            let mut look_from = Point3::new(13.0, 10.0, 10.0);
            let mut look_at = Point3::new(0.0, 0.0, 0.0);
            let mut vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let apeture = 0.1;
            //check scene
            if !args[2].is_empty() {
                match args[2].as_str() {
                    "default" => {
                        look_from = Point3::new(13.0, 10.0, 10.0);
                        look_at = Point3::new(0.0, 0.0, 0.0);
                        vup = Vec3::new(0.0, 1.0, 0.0);
                    }
                    "scene1" => {
                        // Caméra vue de côté gauche
                        look_from = Point3::new(-13.0, 5.0, 3.0);
                        look_at = Point3::new(0.0, 0.0, 0.0);
                        vup = Vec3::new(0.0, 1.0, 0.0);
                    }
                    "scene2" => {
                        // Caméra vue de côté droit
                        look_from = Point3::new(13.0, 5.0, -3.0);
                        look_at = Point3::new(0.0, 0.0, 0.0);
                        vup = Vec3::new(0.0, 1.0, 0.0);
                    }
                    "scene3" => {
                        // Caméra vue de dessus
                        look_from = Point3::new(0.0, 10.0, 0.0);
                        look_at = Point3::new(0.0, 0.0, 0.0);
                        vup = Vec3::new(1.0, 0.0, 0.0); // vecteur vers le haut aligné horizontalement
                    }
                    _ => {
                        actif = true;
                    }
                }
            }
            if actif {
                eprintln!("scene not found");
                return;
            }
            let cam = Camera::camera(
                look_from,
                look_at,
                vup,
                20.0,
                aspect_ratio,
                apeture,
                dist_to_focus
            );
            //choose the shape
            for v in args[1].split_whitespace().into_iter().collect::<Vec<&str>>() {
                match v {
                    "sphere" => {
                        list.push(
                            Box::new(
                                Sphere::sphere(
                                    Point3::new(5.0, 1.5, 0.0),
                                    1.5,
                                    Material::Dielectric {
                                        ref_idx: 1.5,
                                    }
                                )
                            )
                        );
                    }
                    "cylindre" => {
                        let cylinder_material = Material::Lambertian {
                            albedo: Vec3::new(0.8, 0.3, 0.3),
                        };

                        list.push(
                            Box::new(
                                Cylinder::cylinder(
                                    Point3::new(1.0, 0.0, -1.0), // Centre de la base du cylindre
                                    2.0, // Hauteur du cylindre
                                    1.5, // Rayon du cylindre
                                    cylinder_material
                                )
                            )
                        );
                    }
                    "cube" => {
                        let cube_material = Material::Lambertian {
                            albedo: Vec3::new(0.4, 0.4, 0.8),
                        };
                        let cube_min = Point3::new(-3.5, -1.5, 0.0);
                        let cube_max = Point3::new(-0.5, 1.5, 3.5);

                        list.push(Box::new(Cube::new(cube_min, cube_max, cube_material)));
                    }
                    _ => {
                        actif = true;
                        break;
                    }
                }
            }
            if actif {
                eprintln!(
                    "Error shape no identify, Please check between: sphere, cube or cylindre"
                );
                return;
            }
            let world = HittableList::new(list);

            let mut screen = vec![(0u32, 0u32, 0u32); width * height];
            let start = time::Instant::now();

            let bar = ProgressBar::new((width * height) as u64);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({pos}/{len}, ETA {eta})"
                    )
                    .progress_chars("#>-")
            );

            screen
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    let mut rng = rand::thread_rng();
                    let column = index % width;
                    let row = height - index / width;

                    let mut col = Vec3::default();

                    for _ in 0..samples {
                        let u = ((column as f32) + rng.gen::<f32>()) / (width as f32);
                        let v = ((row as f32) + rng.gen::<f32>()) / (height as f32);

                        let r = &cam.get_ray(u, v);
                        col = col + color(&r, &world, 0);
                    }

                    col = col / (samples as f32);
                    col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

                    let ir = (255.99 * col.r()) as u32;
                    let ig = (255.99 * col.g()) as u32;
                    let ib = (255.99 * col.b()) as u32;

                    *pixel = (ir, ig, ib);
                    bar.inc(1);
                });
            bar.finish_with_message("Rendering complete");

            eprintln!("Number of pixels generated: {}", screen.len());

            println!("P3\n{} {}\n{}", width, height, max_value);

            for (r, g, b) in screen {
                println!("{} {} {}", r, g, b);
            }

            let duration = time::Instant::now() - start;
            eprintln!("Render elapsed time: {:?}", duration);
        }
    }
}
