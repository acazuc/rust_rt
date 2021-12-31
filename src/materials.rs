use crate::ray::Ray;

use crate::math::{Vec3d,Vec2d};

use crate::hittable::HitRecord;

use crate::textures::Texture;

use std::sync::Arc;

use rand::Rng;

pub trait Material: Send + Sync
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>;
	fn emitted(&self, _uv: Vec2d, _p: Vec3d) -> Vec3d
	{
		Vec3d::newv(0.0)
	}
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

		Some((self.albedo.value(rec.uv, rec.p), Ray::with_time(rec.p, scatter_direction, r.time())))
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
		let reflected = Vec3d::reflect(Vec3d::normalize(r.dir()), rec.normal);
		let scattered = Ray::with_time(rec.p, reflected + Vec3d::random_in_unit_sphere() * self.fuzz, r.time());
		if Vec3d::dot(scattered.dir(), rec.normal) > 0.0
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

pub struct DiffuseLight
{
	emit: Arc::<dyn Texture>,
}

impl DiffuseLight
{
	pub fn new(emit: Arc::<dyn Texture>) -> Self
	{
		Self{emit}
	}
}

impl Material for DiffuseLight
{
	fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		None
	}

	fn emitted(&self, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		self.emit.value(uv, p)
	}
}
