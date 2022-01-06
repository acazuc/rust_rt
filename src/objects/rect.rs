use crate::hittable::
{
	HitRecord,
	Hittable,
};
use crate::materials::Material;
use crate::math::
{
	aabb::Aabb,
	vec::
	{
		Vec2d,
		Vec3d,
	}
};
use crate::ray::Ray;
use crate::scene::Scene;

use rand::Rng;

use std::sync::Arc;

pub struct XYRect
{
	x0: f64,
	x1: f64,
	y0: f64,
	y1: f64,
	k: f64,
	material: Arc::<dyn Material>,
}

impl XYRect
{
	pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc::<dyn Material>) -> Self
	{
		Self{x0, x1, y0, y1, k, material}
	}
}

impl Hittable for XYRect
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let t = (self.k - r.orig().z()) / r.dir().z();
		if t < tmin || t > tmax
		{
			return None;
		}

		let x = r.orig().x() + t * r.dir().x();
		let y = r.orig().y() + t * r.dir().y();
		if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1
		{
			return None;
		}

		let uv = Vec2d::new((x - self.x0) / (self.x1 - self.x0),
		                    (y - self.y0) / (self.y1 - self.y0));
		let norm = Vec3d::new(0.0, 0.0, 1.0);
		let p = r.at(t);
		Some(HitRecord::new(r, p, t, uv, norm, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(Vec3d::new(self.x0, self.y0, self.k - std::f64::EPSILON),
		               Vec3d::new(self.x1, self.y1, self.k + std::f64::EPSILON)))
	}
}

pub struct XZRect
{
	x0: f64,
	x1: f64,
	z0: f64,
	z1: f64,
	k: f64,
	material: Arc::<dyn Material>,
}

impl XZRect
{
	pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc::<dyn Material>) -> Self
	{
		Self{x0, x1, z0, z1, k, material}
	}
}

impl Hittable for XZRect
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let t = (self.k - r.orig().y()) / r.dir().y();
		if t < tmin || t > tmax
		{
			return None;
		}

		let x = r.orig().x() + t * r.dir().x();
		let z = r.orig().z() + t * r.dir().z();
		if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1
		{
			return None;
		}

		let uv = Vec2d::new((x - self.x0) / (self.x1 - self.x0),
		                    (z - self.z0) / (self.z1 - self.z0));
		let norm = Vec3d::new(0.0, 1.0, 0.0);
		let p = r.at(t);
		Some(HitRecord::new(r, p, t, uv, norm, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(Vec3d::new(self.x0, self.k - std::f64::EPSILON, self.z0),
		               Vec3d::new(self.x1, self.k + std::f64::EPSILON, self.z1)))
	}

	fn pdf_value(&self, o: Vec3d, v: Vec3d) -> f64
	{
		if let Some(rec) = self.hit(&Ray::new(o, v), 0.001, f64::INFINITY)
		{
			let area = (self.x1 - self.x0)  * (self.z1 - self.z0);
			let distance_squared = rec.t * rec.t * Vec3d::dot(v, v);
			let cosine = f64::abs(Vec3d::dot(v, rec.normal) / Vec3d::length(v));
			return distance_squared / (cosine * area);
		}

		0.0
	}

	fn random(&self, o: Vec3d) -> Vec3d
	{
		let mut rng = rand::thread_rng();
		let random_point = Vec3d::new(rng.gen_range(self.x0..self.x1), self.k, rng.gen_range(self.z0..self.z1));
		random_point - o
	}
}

pub struct YZRect
{
	y0: f64,
	y1: f64,
	z0: f64,
	z1: f64,
	k: f64,
	material: Arc::<dyn Material>,
}

impl YZRect
{
	pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc::<dyn Material>) -> Self
	{
		Self{y0, y1, z0, z1, k, material}
	}
}

impl Hittable for YZRect
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let t = (self.k - r.orig().x()) / r.dir().x();
		if t < tmin || t > tmax
		{
			return None;
		}

		let y = r.orig().y() + t * r.dir().y();
		let z = r.orig().z() + t * r.dir().z();
		if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1
		{
			return None;
		}

		let uv = Vec2d::new((y - self.y0) / (self.y1 - self.y0),
		                    (z - self.z0) / (self.z1 - self.z0));
		let norm = Vec3d::new(1.0, 0.0, 0.0);
		let p = r.at(t);
		Some(HitRecord::new(r, p, t, uv, norm, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(Vec3d::new(self.k - std::f64::EPSILON, self.y0, self.z0),
		               Vec3d::new(self.k + std::f64::EPSILON, self.y1, self.z1)))
	}
}
