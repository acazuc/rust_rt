use crate::math::Vec3d;

use std::sync::Arc;

pub trait Texture: Sync + Send
{
	fn value(&self, u: f64, v: f64, p: Vec3d) -> Vec3d;
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
	fn value(&self, u: f64, v: f64, p: Vec3d) -> Vec3d
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
	fn value(&self, u: f64, v: f64, p: Vec3d) -> Vec3d
	{
		let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
		if sines < 0.0
		{
			self.odd.value(u, v, p)
		}
		else
		{
			self.even.value(u, v, p)
		}
	}
}
