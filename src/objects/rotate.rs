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

use std::sync::Arc;

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
							let x = i as f64 * aabb.max().x() + (1 - i) as f64 * aabb.min().x();
							let y = j as f64 * aabb.max().y() + (1 - j) as f64 * aabb.min().y();
							let z = k as f64 * aabb.max().z() + (1 - k) as f64 * aabb.min().z();

							let newx =  cos_theta * x + sin_theta * z;
							let newz = -sin_theta * x + cos_theta * z;

							let tester = Vec3d::new(newx, y, newz);
							for c in 0..3
							{
								min.v[c] = f64::min(min.v[c], tester.v[c]);
								max.v[c] = f64::max(max.v[c], tester.v[c]);
							}
						}
					}
				}

				Some(Aabb::new(min, max))
			}
		};

		Self{obj: obj.clone(), sin_theta, cos_theta, aabb}
	}
}

impl Hittable for RotateY
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let mut orig = r.orig();
		let mut dir = r.dir();

		orig.v[0] = self.cos_theta * r.orig().x() - self.sin_theta * r.orig().z();
		orig.v[2] = self.sin_theta * r.orig().x() + self.cos_theta * r.orig().z();

		dir.v[0] = self.cos_theta * r.dir().x() - self.sin_theta * r.dir().z();
		dir.v[2] = self.sin_theta * r.dir().x() + self.cos_theta * r.dir().z();

		let rotated_r = Ray::with_time(orig, dir, r.time());

		if let Some(mut rec) = self.obj.hit(&rotated_r, tmin, tmax)
		{
			let mut p = rec.p;
			let mut normal = rec.normal;

			p.v[0] =  self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
			p.v[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

			normal.v[0] =  self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
			normal.v[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

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
}
