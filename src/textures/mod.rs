pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;
pub mod solid_color;

use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::scene::Scene;

use std::sync::Arc;

pub trait Texture: Sync + Send
{
	fn value(&self, uv: Vec2d, p: Vec3d) -> Vec3d;
}
