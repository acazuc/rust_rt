use crate::hittable::
{
	HitRecord,
	Hittable,
};
use crate::materials::
{
	Material,
	isotropic::Isotropic,
};
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
use crate::textures::Texture;

use rand::Rng;

use std::sync::Arc;

pub struct ConstantMedium
{
	boundary: Arc::<dyn Hittable>,
	phase_function: Arc::<dyn Material>,
	neg_inv_density: f64,
}

impl ConstantMedium
{
	pub fn new(b: Arc::<dyn Hittable>, d: f64, a: Arc::<dyn Texture>) -> Self
	{
		Self{boundary: b, phase_function: Arc::new(Isotropic::new(a)), neg_inv_density: -1.0 / d}
	}
}

impl Hittable for ConstantMedium
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let mut rng = rand::thread_rng();

		if let Some(mut rec1) = self.boundary.hit(r, -std::f64::INFINITY, std::f64::INFINITY)
		{
			if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, std::f64::INFINITY)
			{
				if rec1.t < tmin
				{
					rec1.t = tmin;
				}

				if rec2.t > tmax
				{
					rec2.t = tmax;
				}

				if rec1.t >= rec2.t
				{
					return None;
				}

				if rec1.t < 0.0
				{
					rec1.t = 0.0;
				}

				let ray_length = Vec3d::length(r.dir());
				let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
				let hit_distance = self.neg_inv_density * f64::log(rng.gen_range(0.0..1.0), 2.0);

				if hit_distance > distance_inside_boundary
				{
					return None;
				}

				let t = rec1.t + hit_distance / ray_length;
				let p = r.at(t);
				return Some(HitRecord::new(r, p, t, Vec2d::zero(), Vec3d::new(1.0, 0.0, 0.0), self.phase_function.clone()));
			}
		}

		None
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		self.boundary.bounding_box(time0, time1)
	}
}
