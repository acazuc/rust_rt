mod bvh;
mod camera;
mod hittable;
mod math;
mod materials;
mod objects;
mod onb;
mod perlin;
mod pdf;
mod ray;
mod scene;
mod textures;

use crate::hittable::Hittable;
use crate::math::vec::Vec3d;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::pdf::
{
	Pdf,
	cosine_pdf::CosinePdf,
	hittable_pdf::HittablePdf,
	mixture_pdf::MixturePdf,
};

use indicatif::
{
	ProgressBar,
	ProgressStyle,
};

use rand::Rng;

use std::sync::Arc;

use rayon::prelude::*;

use std::
{
	sync::
	{
		Mutex,
		mpsc::
		{
			channel,
			Sender,
			Receiver,
		},
	},
	thread,
	time::Instant,
};

struct Pixel
{
	pub x: u32,
	pub y: u32,
	pub color: Vec3d,
}

impl Pixel
{
	pub fn new(x: u32, y: u32, color: Vec3d) -> Self
	{
		Self{x, y, color}
	}
}

fn ray_color(r: &Ray, background: Vec3d, objects: Arc::<dyn Hittable>, lights: Arc::<dyn Hittable>, depth: i32) -> Vec3d
{
	if depth <= 0
	{
		return Vec3d::newv(0.0);
	}

	if let Some(rec) = objects.hit(r, 0.001, f64::INFINITY)
	{
		let emitted = rec.material.emitted(&r, &rec, rec.uv, rec.p);
		if let Some(tuple) = rec.material.scatter(r, &rec)
		{
			let light_pdf: Arc::<dyn Pdf> = Arc::new(HittablePdf::new(lights.clone(), rec.p));
			let cosine_pdf: Arc::<dyn Pdf> = Arc::new(CosinePdf::new(rec.normal));
			let mixed_pdf = MixturePdf::new(light_pdf.clone(), cosine_pdf.clone());
			let scattered = Ray::with_time(rec.p, mixed_pdf.generate(), r.time());
			let pdf = mixed_pdf.value(scattered.dir());
			return emitted + tuple.0 * rec.material.scattering_pdf(r, &rec, &scattered) * ray_color(&scattered, background, objects, lights, depth - 1) / pdf;
		}

		return emitted
	}

	background
}

fn main()
{
	let samples = 10000;
	let max_depth: i32 = 50;

	//let background = Vec3d::new(0.5, 0.5, 0.75);
	//let background = Vec3d::new(0.02, 0.02, 0.05);
	let background = Vec3d::zero();

	let pb = ProgressBar::new(1);
	pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}, ETA: {eta}] {wide_bar:} {msg}"));

	//let scene = Scene::random_scene();
	//let scene = Scene::simple_light_scene();
	let scene = Scene::cornell_box();
	//let scene = Scene::chapter2();
	//let scene = Scene::from_file("scene/cornell.xml");

	let width = scene.camera().width();
	let height = scene.camera().height();
	let mut pixels: Vec<Vec3d> = vec![Vec3d::newv(0.0); (width * height) as usize];
	let mut imgbuf = image::ImageBuffer::new(width, height);
	let mut pixbuf: Vec<u8> = vec![0 as u8; (width * height * 3) as usize];

	let (chan_sender, chan_receiver): (Sender<Pixel>, Receiver<Pixel>) = channel();
	let shared_sender = Mutex::new(chan_sender);
	let window = show_image::make_window("image").unwrap();

	thread::spawn(move||
	{
		let mut last_display = Instant::now();
		while let Result::Ok(pixel) = chan_receiver.recv()
		{
			let color = pixel.color * 255.0;
			let idx = (pixel.x + width * (height - 1 - pixel.y)) as usize;
			pixbuf[idx * 3 + 0] = (color.x() as i32 & 255) as u8;
			pixbuf[idx * 3 + 1] = (color.y() as i32 & 255) as u8;
			pixbuf[idx * 3 + 2] = (color.z() as i32 & 255) as u8;
			if last_display.elapsed().as_millis() > 100
			{
				last_display = Instant::now();
				let image = (pixbuf.clone(), show_image::ImageInfo::rgb8(width as usize, height as usize));
				window.set_image(image, "image-001").unwrap();
			}
		}
	});

	pb.set_message("rendering");
	pb.set_length((scene.camera().width() * scene.camera().height()) as u64);
	pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)|
	{
		let mut rng = rand::thread_rng();

		let y = idx as u32 / scene.camera().width();
		let x = idx as u32 % scene.camera().width();

		let mut color = Vec3d::zero();

		if true
		{
			for _ in 0..samples
			{
				let u = (x as f64 + rng.gen_range(0.0..1.0)) / (scene.camera().width() - 1) as f64;
				let v = (y as f64 + rng.gen_range(0.0..1.0)) / (scene.camera().height() - 1) as f64;
				let ray = scene.camera().get_ray(u, v);
				color += ray_color(&ray, background, scene.objects(), scene.lights(), max_depth);
			}
			color /= samples as f64;
			color = color.for_each(&f64::sqrt);
			color = Vec3d::min(Vec3d::newv(1.0), Vec3d::max(Vec3d::newv(0.0), color));
		}
		else
		{
			let u = (x as f64 + rng.gen_range(0.0..1.0)) / (scene.camera().width() - 1) as f64;
			let v = (y as f64 + rng.gen_range(0.0..1.0)) / (scene.camera().height() - 1) as f64;
			let ray = scene.camera().get_ray(u, v);
			let bvh_depth = scene.objects().bvh_depth(&ray, 0.001, f64::INFINITY);
			color = match bvh_depth
			{
				None => Vec3d::zero(),
				Some((_t, depth)) => Vec3d::newv(depth as f64 / 10.0),
			};
		}

		shared_sender.lock().unwrap().send(Pixel::new(x, y, color)).unwrap();
		*pixel = color;

		pb.inc(1);
	});

	println!("");
	pb.set_message("converting");
	pb.set_length((scene.camera().width() * scene.camera().height()) as u64);
	pb.reset();
	pixels.iter().enumerate().for_each(|(idx, pix)|
	{
		let y = idx as u32 / scene.camera().width();
		let x = idx as u32 % scene.camera().width();

		let data = *pix * 255.0;
		let pixel = imgbuf.get_pixel_mut(x, scene.camera().height() - y - 1);
		*pixel = image::Rgb([data.x() as u8, data.y() as u8, data.z() as u8]);

		pb.inc(1);
	});

	println!("");
	imgbuf.save("output.png").unwrap();
}
