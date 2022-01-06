pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod mtl;

use crate::hittable::HitRecord;
use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::ray::Ray;
use crate::scene::Scene;

use std::sync::Arc;

pub trait Material: Send + Sync
{
	fn resolve(&self, scene: &Scene)
	{
	}

	fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Vec3d, Ray, f64)>
	{
		None
	}

	fn scattering_pdf(&self, _r: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64
	{
		0.0
	}

	fn emitted(&self, _r: &Ray, _rec: &HitRecord, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		Vec3d::zero()
	}
}
