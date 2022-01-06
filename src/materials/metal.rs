use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::textures::Texture;

use std::sync::Arc;

use super::Material;

pub struct Metal
{
	albedo: Arc::<dyn Texture>,
	fuzz: f64,
}

impl Metal
{
	pub fn new(albedo: Arc::<dyn Texture>, fuzz: f64) -> Self
	{
		Self{albedo, fuzz}
	}
}

impl Material for Metal
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray, f64)>
	{
		let reflected = Vec3d::reflect(Vec3d::normalize(r.dir()), rec.normal);
		let scattered = Ray::with_time(rec.p, reflected + Vec3d::random_in_unit_sphere() * self.fuzz, r.time());
		if Vec3d::dot(scattered.dir(), rec.normal) > 0.0
		{
			return Some((self.albedo.value(rec.uv, rec.p), scattered, 1.0));
		}

		None
	}
}
