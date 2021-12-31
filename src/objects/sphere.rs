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

use std::sync::Arc;

pub struct Sphere
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Sphere
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Sphere{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Sphere
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center;
		let a = Vec3d::dot(r.dir(), r.dir());
		let half_b = Vec3d::dot(oc, r.dir());
		let c = Vec3d::dot(oc, oc) - self.radius * self.radius;
		let d = half_b * half_b - a * c;
		if d < 0.0
		{
			return None;
		}

		let sqrt_d = f64::sqrt(d);
		let mut t = (-half_b - sqrt_d) / a;
		if t < tmin || t > tmax
		{
			t = (-half_b + sqrt_d) / a;
			if t < tmin || t > tmax
			{
				return None;
			}
		}

		let p = r.at(t);
		let normal = (p - self.center) / self.radius;
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}
