use crate::hittable::
{
	HitRecord,
	Hittable,
};
use crate::math::
{
	aabb::Aabb,
	vec::Vec3d,
};
use crate::ray::Ray;
use crate::scene::Scene;

use std::sync::Arc;

pub struct FlipFace
{
	obj: Arc::<dyn Hittable>,
}

impl FlipFace
{
	pub fn new(obj: Arc::<dyn Hittable>) -> Self
	{
		Self{obj}
	}
}

impl Hittable for FlipFace
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		if let Some(mut rec) = self.obj.hit(&r, tmin, tmax)
		{
			rec.front_face = !rec.front_face;
			return Some(rec);
		}

		None
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		self.obj.bounding_box(time0, time1)
	}

	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		self.obj.bvh_depth(&r, tmin, tmax)
	}
}
