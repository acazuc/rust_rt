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

pub struct MovingSphere
{
	center0: Vec3d,
	center1: Vec3d,
	time0: f64,
	time1: f64,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl MovingSphere
{
	pub fn new(center0: Vec3d, center1: Vec3d, time0: f64, time1: f64, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		MovingSphere{center0, center1, time0, time1, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}

	fn center(&self, time: f64) -> Vec3d
	{
		self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
	}
}

impl Hittable for MovingSphere
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center(r.time());
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
		let normal = (p - self.center(r.time())) / self.radius;
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		let box0 = Aabb::new(self.center(time0) - Vec3d::newv(self.radius),
		                     self.center(time0) + Vec3d::newv(self.radius));
		let box1 = Aabb::new(self.center(time1) - Vec3d::newv(self.radius),
		                     self.center(time1) + Vec3d::newv(self.radius));
		Some(Aabb::surrounding_box(&box0, &box1))
	}
}
