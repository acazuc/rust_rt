use crate::math::vec::Vec3d;
use crate::onb::Onb;

use super::Pdf;

pub struct CosinePdf
{
	uvw: Onb,
}

impl CosinePdf
{
	pub fn new(w: Vec3d) -> Self
	{
		Self{uvw: Onb::from_w(w)}
	}
}

impl Pdf for CosinePdf
{
	fn value(&self, direction: Vec3d) -> f64
	{
		let cosine = Vec3d::dot(Vec3d::normalize(direction), self.uvw.w());
		if cosine < 0.0
		{
			0.0
		}
		else
		{
			cosine / std::f64::consts::PI
		}
	}

	fn generate(&self) -> Vec3d
	{
		self.uvw.local(Vec3d::random_cosine_direction())
	}
}
