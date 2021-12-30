use crate::ray::Ray;

use crate::math::Vec3d;

use crate::hittable::HitRecord;

use crate::textures::Texture;

use std::sync::Arc;

use rand::Rng;

pub trait Material: Send + Sync
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>;
}

pub struct Lambertian
{
	albedo: Arc::<dyn Texture>,
}

impl Lambertian
{
	pub fn new(albedo: Arc::<dyn Texture>) -> Self
	{
		Lambertian{albedo}
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

		Some((self.albedo.value(rec.u, rec.v, rec.p), Ray::with_time(rec.p, scatter_direction, r.time())))
	}
}

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
		let reflected = Vec3d::unit_vector(&r.dir()).reflect(&rec.normal);
		let scattered = Ray::with_time(rec.p, reflected + Vec3d::random_in_unit_sphere() * self.fuzz, r.time());
		if scattered.dir().dot(&rec.normal) > 0.0
		{
			return Some((self.albedo, scattered));
		}

		None
	}
}

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

		let unit_direction = Vec3d::unit_vector(&r.dir());
		let munit_direction = -unit_direction;

		let cos_theta = f64::min(rec.normal.dot(&munit_direction), 1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

		let direction;
		if refraction_ratio * sin_theta > 1.0
		{
			direction = unit_direction.reflect(&rec.normal);
		}
		else
		{
			let mut rng = rand::thread_rng();
			if Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
			{
				direction = unit_direction.reflect(&rec.normal);
			}
			else
			{
				direction = unit_direction.refract(&rec.normal, refraction_ratio);
			}
		}

		Some((Vec3d::newv(1.0), Ray::with_time(rec.p, direction, r.time())))
	}
}