pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::ray::Ray;

pub trait Material: Send + Sync
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray)>;
	fn emitted(&self, _uv: Vec2d, _p: Vec3d) -> Vec3d
	{
		Vec3d::newv(0.0)
	}
}
