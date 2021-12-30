use crate::hittable::Hittable;
use crate::hittable::HitRecord;

use crate::math::Vec3d;

use crate::ray::Ray;

use crate::materials::Material;

use std::sync::Arc;

use crate::aabb::Aabb;

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

	pub fn get_uv(&self, p: &Vec3d) -> (f64, f64)
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
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
		let outward_normal = (p - self.center) / self.radius;
		let (u, v) = self.get_uv(&outward_normal);
		let mut record = HitRecord::new(p, t, u, v, self.material.clone());
		record.set_face_normal(r, &outward_normal);
		Some(record)
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}

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

	pub fn get_uv(&self, p: &Vec3d) -> (f64, f64)
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
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
		let outward_normal = (p - self.center(r.time())) / self.radius;
		let (u, v) = self.get_uv(&outward_normal);
		let mut record = HitRecord::new(p, t, u, v, self.material.clone());
		record.set_face_normal(r, &outward_normal);
		Some(record)
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

	pub fn get_uv(&self, p: &Vec3d) -> (f64, f64)
	{
		let theta = f64::acos(0.0);
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
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
		let mut outward_normal = (p - self.center) / self.radius;
		outward_normal = Vec3d::new(outward_normal.x(), 0.0, outward_normal.z());
		let (u, v) = self.get_uv(&outward_normal);
		let mut record = HitRecord::new(p, t, u, v, self.material.clone());
		record.set_face_normal(r, &outward_normal);
		Some(record)
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}

pub struct Cone
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Cone
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Cone{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> (f64, f64)
	{
		let theta = f64::acos(p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Cone
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center;
		let oc2 = Vec3d::new(oc.x(), -oc.y(), oc.z());
		let rdir = r.dir();
		let rdir2 = Vec3d::new(rdir.x(), -rdir.y(), rdir.z());
		let a = Vec3d::dot(rdir2, rdir);
		let half_b = Vec3d::dot(oc, rdir2);
		let c = Vec3d::dot(oc2, oc) - self.radius * self.radius;
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
		let mut outward_normal = (p - self.center) / self.radius;
		outward_normal = Vec3d::new(outward_normal.x(), -outward_normal.y(), outward_normal.z());
		let (u, v) = self.get_uv(&outward_normal);
		let mut record = HitRecord::new(p, t, u, v, self.material.clone());
		record.set_face_normal(r, &outward_normal);
		Some(record)
	}

	fn bounding_box(&self, _time: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}
