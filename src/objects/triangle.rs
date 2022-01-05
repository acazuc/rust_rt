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

pub struct Triangle
{
	p: [Vec3d; 3],
	e: [Vec3d; 2],
	uv: [Vec2d; 3],
	norm: [Vec3d; 3],
	material: Arc::<dyn Material>,
}

impl Triangle
{
	pub fn new(p: [Vec3d; 3], uv: [Vec2d; 3], norm: [Vec3d; 3], material: Arc::<dyn Material>) -> Self
	{
		let e = [p[1] - p[0], p[2] - p[0]];
		Self{p, e, uv, norm, material}
	}

	fn get_uv(&self, u: f64, v: f64) -> Vec2d
	{
		self.uv[1] * u + self.uv[2] * v + self.uv[0] * (1.0 - u - v)
	}

	fn get_norm(&self, u: f64, v: f64) -> Vec3d
	{
		self.norm[1] * u + self.norm[2] * v + self.norm[0] * (1.0 - u - v)
	}
}

impl Hittable for Triangle
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let p = Vec3d::cross(r.dir(), self.e[1]);
		let mut det = Vec3d::dot(self.e[0], p);
		if det > -std::f64::EPSILON && det < std::f64::EPSILON
		{
			return None;
		}

		det = 1.0 / det;
		let tt = r.orig() - self.p[0];
		let u = Vec3d::dot(tt, p) * det;
		if u < std::f64::EPSILON || u > 1.0 + std::f64::EPSILON
		{
			return None;
		}

		let q = Vec3d::cross(tt, self.e[0]);
		let v = Vec3d::dot(r.dir(), q) * det;
		if v < std::f64::EPSILON || u + v > 1.0 + std::f64::EPSILON
		{
			return None;
		}

		let t = Vec3d::dot(self.e[1], q) * det;
		if t < tmin || t > tmax
		{
			return None;
		}

		let p = r.at(t);
		Some(HitRecord::new(r, p, t, self.get_uv(u, v), self.get_norm(u, v), self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new
		(
			Vec3d::new
			(
				f64::min(self.p[0].x(), f64::min(self.p[1].x(), self.p[2].x())),
				f64::min(self.p[0].y(), f64::min(self.p[1].y(), self.p[2].y())),
				f64::min(self.p[0].z(), f64::min(self.p[1].z(), self.p[2].z()))
			),
			Vec3d::new
			(
				f64::max(self.p[0].x(), f64::max(self.p[1].x(), self.p[2].x())),
				f64::max(self.p[0].y(), f64::max(self.p[1].y(), self.p[2].y())),
				f64::max(self.p[0].z(), f64::max(self.p[1].z(), self.p[2].z()))
			)
		))
	}
}
