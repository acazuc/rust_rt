use crate::hittable::HitRecord;
use crate::math::vec::Vec3d;
use crate::onb::Onb;
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
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray, f64)>
	{
		/*let mut scatter_direction = rec.normal + Vec3d::random_unit_vector();
		if scatter_direction.near_zero()
		{
			scatter_direction = rec.normal;
		}

		scatter_direction = Vec3d::normalize(scatter_direction);
		let pdf = Vec3d::dot(rec.normal, scatter_direction) / std::f64::consts::PI;
		Some((self.albedo.value(rec.uv, rec.p), Ray::with_time(rec.p, scatter_direction, r.time()), pdf))*/
		let uvw = Onb::from_w(rec.normal);
		let direction = Vec3d::normalize(uvw.local(Vec3d::random_cosine_direction()));
		Some((self.albedo.value(rec.uv, rec.p),
		      Ray::with_time(rec.p, direction, r.time()),
		      Vec3d::dot(uvw.w(), direction) / std::f64::consts::PI))
	}

	fn scattering_pdf(&self, r: &Ray, rec: &HitRecord, scattered: &Ray) -> f64
	{
		let cosine = Vec3d::dot(rec.normal, Vec3d::normalize(scattered.dir()));
		if cosine < 0.0
		{
			0.0
		}
		else
		{
			cosine / std::f64::consts::PI
		}
	}
}
