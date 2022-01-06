pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;

use crate::math::vec::Vec3d;

pub trait Pdf
{
	fn value(&self, direction: Vec3d) -> f64;
	fn generate(&self) -> Vec3d;
}
