use crate::math::vec::Vec3d;

pub struct Ray
{
	orig: Vec3d,
	dir: Vec3d,
	tm: f64,
}

impl Ray
{
	pub fn new(orig: Vec3d, dir: Vec3d) -> Self
	{
		Ray::with_time(orig, dir, 0.0)
	}

	pub fn with_time(orig: Vec3d, dir: Vec3d, tm: f64) -> Self
	{
		Self{orig, dir, tm}
	}

	pub fn at(&self, t: f64) -> Vec3d
	{
		self.orig + self.dir * t
	}

	pub fn orig(&self) -> Vec3d
	{
		self.orig
	}

	pub fn dir(&self) -> Vec3d
	{
		self.dir
	}

	pub fn time(&self) -> f64
	{
		self.tm
	}
}
