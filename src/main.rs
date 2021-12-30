mod math;
mod ray;
mod objects;
mod hittable;
mod camera;
mod materials;
mod aabb;
mod bvh;
mod textures;

use math::Vec3d;
use ray::Ray;
use hittable::{Hittable, HittableList};
use objects::{Sphere,MovingSphere,Cylinder,Cone};
use materials::{Material,Lambertian,Metal,Dielectric};
use camera::Camera;
use rand::Rng;
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ProgressBar,ProgressStyle};
use aabb::Aabb;
use textures::{SolidColor,CheckerTexture};
use bvh::BvhNode;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3d
{
	if depth <= 0
	{
		return Vec3d::newv(0.0);
	}

	if let Some(rec) = world.hit(r, 0.001, f64::INFINITY)
	{
		if let Some(tuple) = rec.material.scatter(r, &rec)
		{
			return tuple.0 * ray_color(&tuple.1, world, depth - 1);
		}

		return Vec3d::newv(0.0);
	}

	let unit_direction = Vec3d::unit_vector(&r.dir());
	let t: f64 = 0.5 * (unit_direction.y() + 1.0);
	Vec3d::newv(1.0) * (1.0 - t) + Vec3d::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList
{
	let mut rng = rand::thread_rng();
	let mut world = HittableList::new();
	let mut objects: Vec::<Box::<dyn Hittable>> = Vec::new();

	let ground_material: Arc::<dyn Material> = Arc::new(Lambertian::new(
		Arc::new(CheckerTexture::new(
			Arc::new(SolidColor::new(Vec3d::new(0.2, 0.3, 0.1))),
			Arc::new(SolidColor::new(Vec3d::new(0.9, 0.9, 0.9)))
	))));
	objects.push(Box::new(Sphere::new(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

	for a in -11..11
	{
		for b in -11..11
		{
			let center = Vec3d::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));

			let tmp = center - Vec3d::new(4.0, 0.2, 0.0);
			if f64::sqrt(tmp.dot(&tmp)) > 0.9
			{
				let choose_mat = rng.gen_range(0.0..1.0);
				if choose_mat < 0.8
				{
					let albedo = Arc::new(SolidColor::new(Vec3d::random(0.0, 1.0) * Vec3d::random(0.0, 1.0)));
					let material: Arc::<dyn Material> = Arc::new(Lambertian::new(albedo));
					let center2 = center + Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0);
					objects.push(Box::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material.clone())));
				}
				else if choose_mat < 0.95
				{
					let albedo = Vec3d::random(0.5, 1.0);
					let fuzz = rng.gen_range(0.0..0.5);
					let material: Arc::<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
					objects.push(Box::new(Sphere::new(center, 0.2, material.clone())));
				}
				else
				{
					let material: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
					objects.push(Box::new(Sphere::new(center, 0.2, material.clone())));
				}
			}
		}
	}

	let m1: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
	objects.push(Box::new(Sphere::new(Vec3d::new(0.0, 1.0, 0.0), 1.0, m1.clone())));

	let m2: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.4, 0.2, 0.1)))));
	objects.push(Box::new(Sphere::new(Vec3d::new(-4.0, 1.0, 0.0), 1.0, m2.clone())));

	let m3: Arc::<dyn Material> = Arc::new(Metal::new(Vec3d::new(0.7, 0.6, 0.5), 0.0));
	objects.push(Box::new(Sphere::new(Vec3d::new(4.0, 1.0, 0.0), 1.0, m3.clone())));

	world.add(Box::new(BvhNode::new(objects, 0.0, 1.0)));

	world
}

fn two_spheres_scene() -> HittableList
{
	let mut rng = rand::thread_rng();
	let mut world = HittableList::new();
	let mut objects: Vec::<Box::<dyn Hittable>> = Vec::new();

	let checker: Arc::<dyn Material> = Arc::new(Lambertian::new(
		Arc::new(CheckerTexture::new(
			Arc::new(SolidColor::new(Vec3d::new(0.2, 0.3, 0.1))),
			Arc::new(SolidColor::new(Vec3d::new(0.9, 0.9, 0.9)))
		))
	));

	objects.push(Box::new(Sphere::new(Vec3d::new(0.0, -10.0, 0.0), 10.0, checker.clone())));
	objects.push(Box::new(Sphere::new(Vec3d::new(0.0,  10.0, 0.0), 10.0, checker.clone())));

	world.add(Box::new(BvhNode::new(objects, 0.0, 1.0)));

	world
}

fn main()
{
	let aspect_ratio: f64 = 16.0 / 9.0;
	let width: u32 = 1920 * 2;
	let height: u32 = (width as f64 / aspect_ratio) as u32;
	let samples = 10;
	let max_depth: i32 = 50;

	let mut imgbuf = image::ImageBuffer::new(width, height);

	let lookfrom = Vec3d::new(13.0, 2.0, 3.0);
	let lookat = Vec3d::new(0.0, 0.0, 0.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.1;
	let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

	let pb = ProgressBar::new(1);
	pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}, ETA: {eta}] {wide_bar:} {msg}"));

	let world = random_scene();
	//let world = two_spheres_scene();

	let mut pixels: Vec<u8> = vec![0; (width * height * 3) as usize];

	pb.set_message("rendering");
	pb.set_length((width * height) as u64);
	pixels.par_chunks_mut(3).into_par_iter().enumerate().for_each(|(idx, pixel)|
	{
		let mut rng = rand::thread_rng();

		let y = idx as u32 / width;
		let x = idx as u32 % width;

		let mut color = Vec3d::newv(0.0);
		for _ in 0..samples
		{
			let u = (x as f64 + rng.gen_range(0.0..1.0)) / (width - 1) as f64;
			let v = (y as f64 + rng.gen_range(0.0..1.0)) / (height - 1) as f64;

			let ray = camera.get_ray(u, v);

			color += ray_color(&ray, &world, max_depth);
		}

		color /= samples as f64;
		color = color.for_each(&f64::sqrt);
		color *= 255.999;

		pixel[0] = color.x() as u8;
		pixel[1] = color.y() as u8;
		pixel[2] = color.z() as u8;

		pb.inc(1);
	});

	pb.set_message("converting");
	pb.set_length((width * height) as u64);
	pb.reset();
	pixels.chunks_mut(3).enumerate().for_each(|(idx, pix)|
	{
		let y = idx as u32 / width;
		let x = idx as u32 % width;

		let pixel = imgbuf.get_pixel_mut(x, height - y - 1);
		*pixel = image::Rgb([pix[0], pix[1], pix[2]]);

		pb.inc(1);
	});

	imgbuf.save("output.png").unwrap();
}
