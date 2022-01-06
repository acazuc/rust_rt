use crate::hittable::HitRecord;
use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::textures::Texture;

use std::sync::Arc;

use super::Material;

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
	fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Vec3d, Ray, f64)>
	{
		None
	}

	fn emitted(&self, _r: &Ray, rec: &HitRecord, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		if rec.front_face
		{
			self.emit.value(uv, p)
		}
		else
		{
			Vec3d::zero()
		}
	}
}
