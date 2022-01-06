use crate::math::vec::Vec3d;
use crate::hittable::Hittable;

use std::sync::Arc;

use super::Pdf;

pub struct HittablePdf
{
	o: Vec3d,
	obj: Arc::<dyn Hittable>,
}

impl HittablePdf
{
	pub fn new(obj: Arc::<dyn Hittable>, o: Vec3d) -> Self
	{
		Self{o, obj}
	}
}

impl Pdf for HittablePdf
{
	fn value(&self, direction: Vec3d) -> f64
	{
		self.obj.pdf_value(self.o, direction)
	}

	fn generate(&self) -> Vec3d
	{
		self.obj.random(self.o)
	}
}
