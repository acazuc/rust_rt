use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::scene::Scene;

use super::Texture;

pub struct SolidColor
{
	color: Vec3d,
}

impl SolidColor
{
	pub fn new(color: Vec3d) -> Self
	{
		Self{color}
	}
}

impl Texture for SolidColor
{
	fn value(&self, _uv: Vec2d, _p: Vec3d) -> Vec3d
	{
		self.color
	}
}
