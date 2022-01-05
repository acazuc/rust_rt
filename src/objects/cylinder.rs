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

use std::sync::Arc;

pub struct Cylinder
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Cylinder
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Cylinder{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(0.0);
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Cylinder
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let mut oc = r.orig() - self.center;
		oc = Vec3d::new(oc.x(), 0.0, oc.z());
		let mut rdir = r.dir();
		rdir = Vec3d::new(rdir.x(), 0.0, rdir.z());
		let a = Vec3d::dot(rdir, rdir);
		let half_b = Vec3d::dot(oc, rdir);
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
		let mut normal = (p - self.center) / self.radius;
		normal = Vec3d::new(normal.x(), 0.0, normal.z());
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}
