mod bvh;
mod camera;
mod hittable;
mod math;
mod materials;
mod objects;
mod perlin;
mod ray;
mod textures;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::
{
	Hittable,
	HittableList
};
use crate::materials::
{
	Material,
	lambertian::Lambertian,
	metal::Metal,
	dielectric::Dielectric,
	diffuse_light::DiffuseLight,
};
use crate::math::vec::Vec3d;
use crate::objects::
{
	sphere::Sphere,
	moving_sphere::MovingSphere,
	wavefront::Wavefront
};
use crate::textures::
{
	checker_texture::CheckerTexture,
	image_texture::ImageTexture,
	noise_texture::NoiseTexture,
	solid_color::SolidColor,
};
use crate::ray::Ray;

use indicatif::
{
	ProgressBar,
	ProgressStyle,
};

use rand::Rng;

use rayon::prelude::*;

use std::sync::Arc;

fn ray_color(r: &Ray, background: Vec3d, world: &dyn Hittable, depth: i32) -> Vec3d
{
	if depth <= 0
	{
		return Vec3d::newv(0.0);
	}

	if let Some(rec) = world.hit(r, 0.001, f64::INFINITY)
	{
		let emitted = rec.material.emitted(rec.uv, rec.p);
		if let Some(tuple) = rec.material.scatter(r, &rec)
		{
			return tuple.0 * ray_color(&tuple.1, background, world, depth - 1);
		}

		return emitted
	}

	background
}

fn random_scene() -> HittableList
{
	let mut rng = rand::thread_rng();
	let mut world = HittableList::new();
	let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

	let ground_material: Arc::<dyn Material> = Arc::new(Lambertian::new(
		Arc::new(CheckerTexture::new(
			Arc::new(SolidColor::new(Vec3d::new(0.2, 0.3, 0.1))),
			Arc::new(SolidColor::new(Vec3d::new(0.9, 0.9, 0.9)))
	))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

	for a in -11..11
	{
		for b in -11..11
		{
			let center = Vec3d::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));

			let tmp = center - Vec3d::new(4.0, 0.2, 0.0);
			if f64::sqrt(Vec3d::dot(tmp, tmp)) > 0.9
			{
				let choose_mat = rng.gen_range(0.0..1.0);
				if choose_mat < 0.2
				{
					let albedo = Arc::new(SolidColor::new(Vec3d::random(0.0, 1.0) * Vec3d::random(0.0, 1.0) * 1.0));
					let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(albedo));
					let center2 = center + Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0);
					objects.push(Arc::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material.clone())));
				}
				else if choose_mat < 0.3
				{
					let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
					objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
				}
				else if choose_mat < 0.6
				{
					let albedo = Arc::new(NoiseTexture::new(Vec3d::random(0.0, 1.0) * Vec3d::random(0.0, 1.0) * 50.0, 4.0));
					let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(albedo));
					objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
				}
				else if choose_mat < 0.8
				{
					let albedo = Vec3d::random(0.5, 1.0);
					let fuzz = rng.gen_range(0.0..0.5);
					let material: Arc::<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
					objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
				}
				else
				{
					let material: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
					objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
				}
			}
		}
	}

	let m1: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
	objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, 1.0, 0.0), 1.0, m1.clone())));

	let m2: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.4, 0.2, 0.1)))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(-4.0, 1.0, 0.0), 1.0, m2.clone())));

	let m3: Arc::<dyn Material> = Arc::new(Metal::new(Vec3d::new(0.7, 0.6, 0.5), 0.8));
	objects.push(Arc::new(Sphere::new(Vec3d::new(4.0, 1.0, 0.0), 1.0, m3.clone())));

	objects.push(Arc::new(Wavefront::new("cessna.obj", Vec3d::new(-3.0, 1.0, 3.0), Vec3d::newv(1.0 / 10.0))));

	world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

	world
}

fn main()
{
	let aspect_ratio: f64 = 16.0 / 9.0;
	let width: u32 = 100;
	let height: u32 = (width as f64 / aspect_ratio) as u32;
	let samples = 20000;
	let max_depth: i32 = 50;

	let mut imgbuf = image::ImageBuffer::new(width, height);

	let lookfrom = Vec3d::new(-13.0, 5.0, 5.0);
	let lookat = Vec3d::new(-2.0, 0.0, 3.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01; //25;
	let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

	//let background = Vec3d::new(0.5, 0.5, 0.75);
	let background = Vec3d::new(0.0, 0.0, 0.0);

	let pb = ProgressBar::new(1);
	pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}, ETA: {eta}] {wide_bar:} {msg}"));

	let world = random_scene();
	//let world = two_spheres_scene();

	let mut pixels: Vec<Vec3d> = vec![Vec3d::newv(0.0); (width * height) as usize];

	pb.set_message("rendering");
	pb.set_length((width * height) as u64);
	pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)|
	{
		let mut rng = rand::thread_rng();

		let y = idx as u32 / width;
		let x = idx as u32 % width;

		for _ in 0..samples
		{
			let u = (x as f64 + rng.gen_range(0.0..1.0)) / (width - 1) as f64;
			let v = (y as f64 + rng.gen_range(0.0..1.0)) / (height - 1) as f64;

			let ray = camera.get_ray(u, v);

			*pixel += ray_color(&ray, background, &world, max_depth);
		}

		*pixel /= samples as f64;
		*pixel = pixel.for_each(&f64::sqrt);

		pb.inc(1);
	});

	pb.set_message("converting");
	pb.set_length((width * height) as u64);
	pb.reset();
	pixels.iter().enumerate().for_each(|(idx, pix)|
	{
		let y = idx as u32 / width;
		let x = idx as u32 % width;

		let pixel = imgbuf.get_pixel_mut(x, height - y - 1);
		*pixel = image::Rgb([(pix.x() * 255.0) as u8, (pix.y() * 255.0) as u8, (pix.z() * 255.0) as u8]);

		pb.inc(1);
	});

	imgbuf.save("output.png").unwrap();
}
