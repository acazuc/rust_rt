use crate::math::Vec3d;

use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Aabb
{
	min: Vec3d,
	max: Vec3d,
}

impl Aabb
{
	pub fn new(min: Vec3d, max: Vec3d) -> Self
	{
		Self{min, max}
	}

	pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool
	{
		let mut min = t_min;
		let mut max = t_max;
		for a in 0..3
		{
			let invd = 1.0 / r.dir().v[a];
			let mut t0 = (self.min.v[a] - r.orig().v[a]) * invd;
			let mut t1 = (self.max.v[a] - r.orig().v[a]) * invd;
			if invd < 0.0
			{
				std::mem::swap(&mut t0, &mut t1);
			}
			min = f64::max(t0, min);
			max = f64::min(t1, max);
			if max < min
			{
				return false;
			}
		}
		true
	}

	pub fn surrounding_box(box0: &Self, box1: &Self) -> Self
	{
		let small = Vec3d::new(f64::min(box0.min().x(), box1.min().x()),
		                       f64::min(box0.min().y(), box1.min().y()),
		                       f64::min(box0.min().z(), box1.min().z()));
		let big = Vec3d::new(f64::max(box0.max().x(), box1.max().x()),
		                     f64::max(box0.max().y(), box1.max().y()),
		                     f64::max(box0.max().z(), box1.max().z()));
		Self::new(small, big)
	}

	pub fn min(&self) -> Vec3d
	{
		self.min
	}

	pub fn max(&self) -> Vec3d
	{
		self.max
	}
}
