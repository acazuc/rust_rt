use crate::math::vec::Vec3d;

use rand::Rng;

use std::sync::Arc;

use super::Pdf;

pub struct MixturePdf
{
	p: [Arc::<dyn Pdf>; 2],
}

impl MixturePdf
{
	pub fn new(p0: Arc::<dyn Pdf>, p1: Arc::<dyn Pdf>) -> Self
	{
		Self{p: [p0, p1]}
	}
}

impl Pdf for MixturePdf
{
	fn value(&self, dir: Vec3d) -> f64
	{
		0.5 * self.p[0].value(dir) + 0.5 * self.p[1].value(dir)
	}

	fn generate(&self) -> Vec3d
	{
		let mut rng = rand::thread_rng();
		if rng.gen_range(0.0..1.0) > 0.5
		{
			self.p[0].generate()
		}
		else
		{
			self.p[1].generate()
		}
	}
}
