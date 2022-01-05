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

pub struct Translate
{
	offset: Vec3d,
	obj: Arc::<dyn Hittable>,
}

impl Translate
{
	pub fn new(obj: Arc::<dyn Hittable>, offset: Vec3d) -> Self
	{
		Self{offset, obj}
	}
}

impl Hittable for Translate
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let moved_r = Ray::with_time(r.orig() - self.offset, r.dir(), r.time());
		if let Some(mut rec) = self.obj.hit(&moved_r, tmin, tmax)
		{
			rec.p += self.offset;
			rec.set_face_normal(&moved_r, rec.normal);
			return Some(rec);
		}

		None
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		if let Some(aabb) = self.obj.bounding_box(time0, time1)
		{
			return Some(Aabb::new(aabb.min() + self.offset,
			                      aabb.max() + self.offset));
		}

		None
	}

	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		let moved_r = Ray::with_time(r.orig() - self.offset, r.dir(), r.time());
		if let Some(rec) = self.obj.bvh_depth(&moved_r, tmin, tmax)
		{
			return Some(rec);
		}

		None
	}
}
