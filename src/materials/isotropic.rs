use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::textures::Texture;

use std::sync::Arc;

use super::Material;

pub struct Isotropic
{
	albedo: Arc::<dyn Texture>,
}

impl Isotropic
{
	pub fn new(albedo: Arc::<dyn Texture>) -> Self
	{
		Self{albedo}
	}
}

impl Material for Isotropic
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		let scattered = Ray::with_time(rec.p, Vec3d::random_in_unit_sphere(), r.time());
		let attenuation = self.albedo.value(rec.uv, rec.p);
		Some((attenuation, scattered))
	}
}
