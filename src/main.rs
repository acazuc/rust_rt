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
	dielectric::Dielectric,
	diffuse_light::DiffuseLight,
	lambertian::Lambertian,
	metal::Metal,
};
use crate::math::vec::Vec3d;
use crate::objects::
{
	constant_medium::ConstantMedium,
	cube::Cube,
	moving_sphere::MovingSphere,
	obj::Obj,
	rect::
	{
		XYRect,
		XZRect,
		YZRect,
	},
	rotate::RotateY,
	sphere::Sphere,
	stl::Stl,
	translate::Translate,
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

fn random_scene(aspect_ratio: f64) -> (HittableList, Camera)
{
	let lookfrom = Vec3d::new(-5.0, 20.0, 5.0);
	let lookat = Vec3d::new(-2.0, 0.0, 3.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01;
	let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

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

	let m3: Arc::<dyn Material> = Arc::new(Metal::new(Vec3d::new(0.7, 0.6, 0.5), 0.0));
	objects.push(Arc::new(Sphere::new(Vec3d::new(4.0, 1.0, 0.0), 1.0, m3.clone())));

	let m4: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::new(0.5, 0.5, 0.8) * 10.0))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(10.0, 10.0, 10.0), 5.0, m4.clone())));

	objects.push(Arc::new(Obj::new("cessna.obj", Vec3d::new(-3.0, 1.0, 3.0), Vec3d::newv(1.0 / 10.0))));
	objects.push(Arc::new(Stl::new("frostmourne.stl", Vec3d::new(0.0, -1.0, 7.0), Vec3d::newv(1.0 / 15.0), m2)));

	world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

	(world, camera)
}

fn simple_light_scene(aspect_ratio: f64) -> (HittableList, Camera)
{
	let lookfrom = Vec3d::new(26.0, 3.0, 6.0);
	let lookat = Vec3d::new(0.0, 2.0, 0.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01;
	let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

	let mut world = HittableList::new();
	let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

	let pertext: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(Vec3d::one(), 4.0))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, pertext.clone())));
	objects.push(Arc::new(Sphere::new(Vec3d::new(0.0,     2.0, 0.0),    2.0, pertext.clone())));

	let difflight: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::newv(4.0)))));
	objects.push(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone())));

	world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

	(world, camera)
}

fn cornell_box(aspect_ratio: f64) -> (HittableList, Camera)
{
	let lookfrom = Vec3d::new(278.0, 278.0, -800.0);
	let lookat = Vec3d::new(278.0, 278.0, 0.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01;
	let camera = Camera::with_time(lookfrom, lookat, vup, 40.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

	let mut world = HittableList::new();
	let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

	let red   = Arc::new(  Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.64, 0.05, 0.05)))));
	let white = Arc::new(  Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.73, 0.73, 0.73)))));
	let green = Arc::new(  Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.12, 0.45, 0.15)))));
	let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::new(7.0 , 7.0 , 7.0)))));

	objects.push(Arc::new(YZRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, green.clone())));
	objects.push(Arc::new(YZRect::new(  0.0, 555.0,   0.0, 555.0,   0.0, red.clone())));
	objects.push(Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));
	objects.push(Arc::new(XZRect::new(  0.0, 555.0,   0.0, 555.0,   0.0, white.clone())));
	objects.push(Arc::new(XZRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, white.clone())));
	objects.push(Arc::new(XYRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, white.clone())));

	let mut box1: Arc::<dyn Hittable> = Arc::new(Cube::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(165.0, 330.0, 165.0), white.clone()));
	box1 = Arc::new(RotateY::new(box1.clone(), 15.0));
	box1 = Arc::new(Translate::new(box1.clone(), Vec3d::new(265.0, 0.0, 295.0)));

	objects.push(Arc::new(ConstantMedium::new(box1.clone(), 0.01, Arc::new(SolidColor::new(Vec3d::zero())))));

	let mut box2: Arc::<dyn Hittable> = Arc::new(Cube::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(165.0, 165.0, 165.0), white.clone()));
	box2 = Arc::new(RotateY::new(box2.clone(), -18.0));
	box2 = Arc::new(Translate::new(box2.clone(), Vec3d::new(130.0, 0.0, 65.0)));
	objects.push(Arc::new(ConstantMedium::new(box2.clone(), 0.01, Arc::new(SolidColor::new(Vec3d::one())))));

	world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

	(world, camera)
}

fn chapter2(aspect_ratio: f64) -> (HittableList, Camera)
{
	let lookfrom = Vec3d::new(478.0, 278.0, -600.0);
	let lookat = Vec3d::new(278.0, 278.0, 0.0);
	let vup = Vec3d::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.01;
	let camera = Camera::with_time(lookfrom, lookat, vup, 40.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

	let mut rng = rand::thread_rng();
	let mut world = HittableList::new();
	let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

	let ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.48, 0.83, 0.53)))));
	let mut boxes1: Vec::<Arc::<dyn Hittable>> = Vec::new();
	let boxes_per_side = 20;
	for i in 0..boxes_per_side
	{
		for j in 0..boxes_per_side
		{
			let w = 100.0;
			let x0 = -1000.0 + i as f64 * w;
			let z0 = -1000.0 + j as f64 * w;
			let y0 = 0.0;
			let x1 = x0 + w;
			let y1 = rng.gen_range(1.0..101.0);
			let z1 = z0 + w;

			boxes1.push(Arc::new(Cube::new(Vec3d::new(x0, y0, z0), Vec3d::new(x1, y1, z1), ground.clone())));
		}
	}
	objects.push(Arc::new(BvhNode::new(boxes1, 0.0, 1.0)));

	let light: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::newv(7.0)))));
	objects.push(Arc::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())));

	let center1 = Vec3d::new(400.0, 400.0, 200.0);
	let center2 = center1 + Vec3d::new(30.0, 0.0, 0.0);
	let moving_sphere_material: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.7, 0.3, 0.1)))));
	objects.push(Arc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone())));
	objects.push(Arc::new(Sphere::new(Vec3d::new(260.0, 140.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Vec3d::new(0.8, 0.8, 0.9), 1.0)))));

	let b1: Arc::<dyn Hittable> = Arc::new(Sphere::new(Vec3d::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
	objects.push(b1.clone());
	objects.push(Arc::new(ConstantMedium::new(b1.clone(), 0.2, Arc::new(SolidColor::new(Vec3d::new(0.2, 0.4, 0.9))))));
	let b2: Arc::<dyn Hittable> = Arc::new(Sphere::new(Vec3d::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
	objects.push(Arc::new(ConstantMedium::new(b2.clone(), 0.0001, Arc::new(SolidColor::new(Vec3d::new(1.0, 1.0, 1.0))))));

	objects.push(Arc::new(Sphere::new(Vec3d::new(400.0, 200.0, 400.0), 100.0, Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg")))))));
	objects.push(Arc::new(Sphere::new(Vec3d::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(Vec3d::new(1.0, 1.0, 1.0), 0.1)))))));

	let mut boxes2: Vec::<Arc::<dyn Hittable>> = Vec::new();
	let white: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.73, 0.73, 0.73)))));
	for _ in 0..1000
	{
		boxes2.push(Arc::new(Sphere::new(Vec3d::random(0.0, 165.0), 10.0, white.clone())));
	}
	objects.push(Arc::new
	(
		Translate::new
		(
			Arc::new(RotateY::new
			(
				Arc::new(BvhNode::new(boxes2, 0.0, 1.0)),
				15.0
			)),
			Vec3d::new(-100.0, 270.0, 395.0)
		)
	));

	world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

	(world, camera)
}

fn main()
{
	let aspect_ratio: f64 = 9.0 / 9.0;
	let width: u32 = 1080;
	let height: u32 = (width as f64 / aspect_ratio) as u32;
	let samples = 1000;
	let max_depth: i32 = 50;

	let mut imgbuf = image::ImageBuffer::new(width, height);

	//let background = Vec3d::new(0.5, 0.5, 0.75);
	//let background = Vec3d::new(0.02, 0.02, 0.05);
	let background = Vec3d::zero();

	let pb = ProgressBar::new(1);
	pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}, ETA: {eta}] {wide_bar:} {msg}"));

	//let (world, camera) = random_scene(aspect_ratio);
	//let (world, camera) = simple_light_scene(aspect_ratio);
	//let (world, camera) = cornell_box(aspect_ratio);
	let (world, camera) = chapter2(aspect_ratio);

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

	println!("");
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

	println!("");
	imgbuf.save("output.png").unwrap();
}
