use crate::math::
{
	aabb::Aabb,
	vec::
	{
		Vec2d,
		Vec3d,
	}
};
use crate::materials::Material;
use crate::ray::Ray;
use crate::scene::Scene;

use std::sync::Arc;

pub struct HitRecord
{
	pub p: Vec3d,
	pub normal: Vec3d,
	pub t: f64,
	pub uv: Vec2d,
	pub front_face: bool,
	pub material: Arc::<dyn Material>,
}

impl HitRecord
{
	pub fn new(r: &Ray, p: Vec3d, t: f64, uv: Vec2d, n: Vec3d, material: Arc::<dyn Material>) -> Self
	{
		let normal;
		let front_face = Vec3d::dot(r.dir(), n) < 0.0;
		if front_face
		{
			normal = n;
		}
		else
		{
			normal = -n;
		}
		Self{p, normal, t, uv, front_face, material}
	}

	pub fn set_face_normal(&mut self, r: &Ray, normal: Vec3d)
	{
		let front_face = Vec3d::dot(r.dir(), normal) < 0.0;
		if front_face
		{
			self.normal = normal;
		}
		else
		{
			self.normal = -normal;
		}
	}
}

pub trait Hittable : Sync + Send
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		if let Some(rec) = self.hit(r, tmin, tmax)
		{
			return Some((rec.t, 1));
		}

		None
	}
}

pub struct HittableList
{
	pub objects: Vec<Arc::<dyn Hittable>>,
}

impl HittableList
{
	pub fn new() -> Self
	{
		HittableList{objects: Vec::default()}
	}

	pub fn push(&mut self, object: Arc::<dyn Hittable>)
	{
		self.objects.push(object)
	}
}

impl Hittable for HittableList
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let mut closest_so_far = tmax;
		let mut rec = None;

		for o in &self.objects
		{
			if let Some(ret) = o.hit(r, tmin, closest_so_far)
			{
				closest_so_far = ret.t;
				rec = Some(ret);
			}
		}

		rec
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		if self.objects.is_empty()
		{
			return None;
		}

		let mut ret: Option<Aabb> = None;
		for o in &self.objects
		{
			if let Some(res) = o.bounding_box(time0, time1)
			{
				ret = match ret
				{
					None => Some(res),
					Some(r) => Some(Aabb::surrounding_box(&r, &res)),
				}
			}
			else
			{
				return None;
			}

		}

		ret
	}

	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		let mut closest_so_far = tmax;
		let mut rec = None;

		for o in &self.objects
		{
			if let Some(ret) = o.bvh_depth(r, tmin, closest_so_far)
			{
				closest_so_far = ret.0;
				rec = Some(ret);
			}
		}

		rec
	}
}
