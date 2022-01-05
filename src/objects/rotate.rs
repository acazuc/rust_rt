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

pub struct RotateZYX
{
	obj: Arc::<dyn Hittable>,
	cos_theta: Vec3d,
	sin_theta: Vec3d,
	aabb: Option<Aabb>,
}

impl RotateZYX
{
	pub fn new(obj: Arc::<dyn Hittable>, angle: Vec3d) -> Self
	{
		let radians = angle.for_each(&f64::to_radians);
		let sin_theta = radians.for_each(&f64::sin);
		let cos_theta = radians.for_each(&f64::cos);
		let aabb = match obj.bounding_box(0.0, 1.0)
		{
			None => None,
			Some(aabb) =>
			{
				let mut min = Vec3d::new( std::f64::INFINITY,  std::f64::INFINITY,  std::f64::INFINITY);
				let mut max = Vec3d::new(-std::f64::INFINITY, -std::f64::INFINITY, -std::f64::INFINITY);

				for i in 0..2
				{
					for j in 0..2
					{
						for k in 0..2
						{
							let v = Vec3d::new
							(
								i as f64 * aabb.max().x() + (1 - i) as f64 * aabb.min().x(),
								j as f64 * aabb.max().y() + (1 - j) as f64 * aabb.min().y(),
								k as f64 * aabb.max().z() + (1 - k) as f64 * aabb.min().z()
							);

							let newv = Self::transform(cos_theta, sin_theta, v);
							for c in 0..3
							{
								min.v[c] = f64::min(min.v[c], newv.v[c]);
								max.v[c] = f64::max(max.v[c], newv.v[c]);
							}
						}
					}
				}

				Some(Aabb::new(min, max))
			}
		};

		Self{obj: obj.clone(), sin_theta, cos_theta, aabb}
	}

	fn transform(c: Vec3d, s: Vec3d, v: Vec3d) -> Vec3d
	{
		Vec3d::new
		(
			v.x() * (c.x() * c.y()) + v.y() * (c.x() * s.y() * s.z() - s.x() * c.z()) + v.z() * (c.x() * s.y() * c.z() + s.x() * s.z()),
			v.x() * (s.x() * c.y()) + v.y() * (s.x() * s.y() * s.z() + c.x() * c.z()) + v.z() * (s.x() * s.y() * c.z() - c.x() * s.z()),
			v.x() * (-s.y())        + v.y() * (c.y() * s.z())                         + v.z() * (c.y() * c.z())
		)
	}

	fn transform_inv(c: Vec3d, s: Vec3d, v: Vec3d) -> Vec3d
	{
		Vec3d::zero()
	}
}

pub struct RotateY
{
	obj: Arc::<dyn Hittable>,
	sin_theta: f64,
	cos_theta: f64,
	aabb: Option<Aabb>,
}

impl RotateY
{
	pub fn new(obj: Arc::<dyn Hittable>, angle: f64) -> Self
	{
		let radians = f64::to_radians(angle);
		let sin_theta = f64::sin(radians);
		let cos_theta = f64::cos(radians);
		let aabb = match obj.bounding_box(0.0, 1.0)
		{
			None => None,
			Some(aabb) =>
			{
				let mut min = Vec3d::new( std::f64::INFINITY,  std::f64::INFINITY,  std::f64::INFINITY);
				let mut max = Vec3d::new(-std::f64::INFINITY, -std::f64::INFINITY, -std::f64::INFINITY);

				for i in 0..2
				{
					for j in 0..2
					{
						for k in 0..2
						{
							let v = Vec3d::new
							(
								i as f64 * aabb.max().x() + (1 - i) as f64 * aabb.min().x(),
								j as f64 * aabb.max().y() + (1 - j) as f64 * aabb.min().y(),
								k as f64 * aabb.max().z() + (1 - k) as f64 * aabb.min().z()
							);

							let newv = Self::transform(cos_theta, sin_theta, v);
							for c in 0..3
							{
								min.v[c] = f64::min(min.v[c], newv.v[c]);
								max.v[c] = f64::max(max.v[c], newv.v[c]);
							}
						}
					}
				}

				Some(Aabb::new(min, max))
			}
		};

		Self{obj: obj.clone(), sin_theta, cos_theta, aabb}
	}

	fn transform(c: f64, s: f64, v: Vec3d) -> Vec3d
	{
		Vec3d::new
		(
			c * v.x() + s * v.z(),
			v.y(),
			-s * v.x() + c * v.z()
		)
	}

	fn transform_inv(c: f64, s: f64, v: Vec3d) -> Vec3d
	{
		Vec3d::new
		(
			c * v.x() - s * v.z(),
			v.y(),
			s * v.x() + c * v.z()
		)
	}
}

impl Hittable for RotateY
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let orig = Self::transform_inv(self.cos_theta, self.sin_theta, r.orig());
		let dir = Self::transform_inv(self.cos_theta, self.sin_theta, r.dir());
		let rotated_r = Ray::with_time(orig, dir, r.time());

		if let Some(mut rec) = self.obj.hit(&rotated_r, tmin, tmax)
		{
			let p = Self::transform(self.cos_theta, self.sin_theta, rec.p);
			let normal = Self::transform(self.cos_theta, self.sin_theta, rec.normal);

			rec.p = p;
			rec.set_face_normal(&rotated_r, normal);
			return Some(rec);
		}

		None
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		self.aabb
	}

	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		let orig = Self::transform_inv(self.cos_theta, self.sin_theta, r.orig());
		let dir = Self::transform_inv(self.cos_theta, self.sin_theta, r.dir());
		let rotated_r = Ray::with_time(orig, dir, r.time());

		if let Some(rec) = self.obj.bvh_depth(&rotated_r, tmin, tmax)
		{
			return Some(rec);
		}

		None
	}
}
