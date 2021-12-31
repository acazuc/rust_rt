use crate::hittable::HitRecord;
use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::ray::Ray;
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
	fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Vec3d, Ray)>
	{
		None
	}

	fn emitted(&self, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		self.emit.value(uv, p)
	}
}
