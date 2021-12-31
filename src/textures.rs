use crate::math::{Vec2d,Vec3d};

use std::sync::Arc;

use crate::perlin::Perlin;

use image::GenericImageView;

pub trait Texture: Sync + Send
{
	fn value(&self, uv: Vec2d, p: Vec3d) -> Vec3d;
}

pub struct SolidColor
{
	color: Vec3d,
}

impl SolidColor
{
	pub fn new(color: Vec3d) -> Self
	{
		Self{color}
	}
}

impl Texture for SolidColor
{
	fn value(&self, _uv: Vec2d, _p: Vec3d) -> Vec3d
	{
		self.color
	}
}

pub struct CheckerTexture
{
	odd: Arc::<dyn Texture>,
	even: Arc::<dyn Texture>,
}

impl CheckerTexture
{
	pub fn new(odd: Arc::<dyn Texture>, even: Arc::<dyn Texture>) -> Self
	{
		Self{odd, even}
	}
}

impl Texture for CheckerTexture
{
	fn value(&self, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
		if sines < 0.0
		{
			self.odd.value(uv, p)
		}
		else
		{
			self.even.value(uv, p)
		}
	}
}

pub struct NoiseTexture
{
	noise: Perlin,
	scale: f64,
	color: Vec3d,
}

impl NoiseTexture
{
	pub fn new(color: Vec3d, scale: f64) -> Self
	{
		Self{noise: Perlin::new(), scale, color}
	}
}

impl Texture for NoiseTexture
{
	fn value(&self, _vu: Vec2d, p: Vec3d) -> Vec3d
	{
		self.color * 0.5 * (1.0 + f64::sin(self.scale * p.z() + self.noise.turb(p * self.scale, 7) * 10.0))
	}
}

pub struct ImageTexture
{
	data: Vec::<u8>,
	width: u32,
	height: u32,
	bytes_per_scanline: u32,
}

impl ImageTexture
{
	pub fn new(filename: &str) -> Self
	{
		let img = image::open(filename).expect("can't load file");

		let (width, height) = img.dimensions();
		let pixels = img.as_flat_samples_u8().expect("can't get data");
		let mut data = Vec::new();
		data.extend_from_slice(pixels.as_slice());
		let bytes_per_scanline = width * 3;

		Self{data, width, height, bytes_per_scanline}
	}
}

impl Texture for ImageTexture
{
	fn value(&self, uv: Vec2d, _p: Vec3d) -> Vec3d
	{
		let uu = f64::clamp(uv.x(), 0.0, 1.0);
		let vv = 1.0 - f64::clamp(uv.y(), 0.0, 1.0);

		let mut i = (uu * self.width as f64) as u32;
		let mut j = (vv * self.height as f64) as u32;

		if i >= self.width
		{
			i = self.width - 1;
		}

		if j >= self.height
		{
			j = self.height - 1;
		}

		let color_scale = 1.0 / 255.0;
		let idx = (j * self.bytes_per_scanline + i * 3) as usize;

		Vec3d::new(self.data[idx + 0] as f64 * color_scale,
		           self.data[idx + 1] as f64 * color_scale,
		           self.data[idx + 2] as f64 * color_scale)
	}
}
