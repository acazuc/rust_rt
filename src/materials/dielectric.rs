use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::ray::Ray;

use rand::Rng;

use super::Material;

pub struct Dielectric
{
	ir: f64,
}

impl Dielectric
{
	pub fn new(ir: f64) -> Self
	{
		Dielectric{ir}
	}

	pub fn reflectance(cosine: f64, ref_idx: f64) -> f64
	{
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
	}
}

impl Material for Dielectric
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		let refraction_ratio;
		if rec.front_face
		{
			refraction_ratio = 1.0 / self.ir;
		}
		else
		{
			refraction_ratio = self.ir;
		}

		let unit_direction = Vec3d::normalize(r.dir());

		let cos_theta = f64::min(Vec3d::dot(rec.normal, -unit_direction), 1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

		let direction;
		if refraction_ratio * sin_theta > 1.0
		{
			direction = Vec3d::reflect(unit_direction, rec.normal);
		}
		else
		{
			let mut rng = rand::thread_rng();
			if Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
			{
				direction = Vec3d::reflect(unit_direction, rec.normal);
			}
			else
			{
				direction = Vec3d::refract(unit_direction, rec.normal, refraction_ratio);
			}
		}

		Some((Vec3d::newv(1.0), Ray::with_time(rec.p, direction, r.time())))
	}
}
