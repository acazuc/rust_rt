use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::perlin::Perlin;

use super::Texture;

pub struct NoiseTexture
{
	noise: Perlin,
	scale: f64,
	color: Vec3d,
}

impl NoiseTexture
{
	pub fn new(color: Vec3d, scale: f64) -> Self
	{
		Self{noise: Perlin::new(), scale, color}
	}
}

impl Texture for NoiseTexture
{
	fn value(&self, _vu: Vec2d, p: Vec3d) -> Vec3d
	{
		self.color * 0.5 * (1.0 + f64::sin(self.scale * p.z() + self.noise.turb(p * self.scale, 7) * 10.0))
	}
}
