use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::ray::Ray;

use super::Material;

pub struct Metal
{
	albedo: Vec3d,
	fuzz: f64,
}

impl Metal
{
	pub fn new(albedo: Vec3d, fuzz: f64) -> Self
	{
		Metal{albedo, fuzz}
	}
}

impl Material for Metal
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		let reflected = Vec3d::reflect(Vec3d::normalize(r.dir()), rec.normal);
		let scattered = Ray::with_time(rec.p, reflected + Vec3d::random_in_unit_sphere() * self.fuzz, r.time());
		if Vec3d::dot(scattered.dir(), rec.normal) > 0.0
		{
			return Some((self.albedo, scattered));
		}

		None
	}
}
