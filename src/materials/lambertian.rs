use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::textures::Texture;

use std::sync::Arc;

use super::Material;

pub struct Lambertian
{
	albedo: Arc::<dyn Texture>,
}

impl Lambertian
{
	pub fn new(albedo: Arc::<dyn Texture>) -> Self
	{
		Self{albedo}
	}
}

impl Material for Lambertian
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		let mut scatter_direction = rec.normal + Vec3d::random_unit_vector();
		if scatter_direction.near_zero()
		{
			scatter_direction = rec.normal;
		}

		Some((self.albedo.value(rec.uv, rec.p), Ray::with_time(rec.p, scatter_direction, r.time())))
	}
}
