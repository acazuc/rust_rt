use crate::math::Vec3d;

use crate::ray::Ray;

use crate::materials::Material;

use crate::aabb::Aabb;

use std::sync::Arc;

pub struct HitRecord
{
	pub p: Vec3d,
	pub normal: Vec3d,
	pub t: f64,
	pub u: f64,
	pub v: f64,
	pub front_face: bool,
	pub material: Arc::<dyn Material>,
}

impl HitRecord
{
	pub fn new(p: Vec3d, t: f64, u: f64, v: f64, material: Arc::<dyn Material>) -> Self
	{
		Self{p: p, normal: Vec3d::default(), t: t, u: u, v: v, front_face: bool::default(), material: material}
	}

	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3d)
	{
		self.front_face = Vec3d::dot(r.dir(), *outward_normal) < 0.0;
		if self.front_face
		{
			self.normal = *outward_normal;
		}
		else
		{
			let a = *outward_normal;
			self.normal = -a;
		}
	}
}

pub trait Hittable : Sync
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub struct HittableList
{
	objects: Vec<Box::<dyn Hittable>>,
}

impl HittableList
{
	pub fn new() -> Self
	{
		HittableList{objects: Vec::default()}
	}

	pub fn add(&mut self, object: Box::<dyn Hittable>)
	{
		self.objects.push(object)
	}

	pub fn clear(&mut self)
	{
		self.objects.clear()
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
}
