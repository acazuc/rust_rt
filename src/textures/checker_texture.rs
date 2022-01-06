use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::scene::Scene;

use std::sync::Arc;

use super::Texture;

pub struct CheckerTexture
{
	odd: Arc::<dyn Texture>,
	even: Arc::<dyn Texture>,
}

impl CheckerTexture
{
	pub fn new(odd: Arc::<dyn Texture>, even: Arc::<dyn Texture>) -> Self
	{
		Self{odd, even}
	}

	/*pub fn from(scene: &Scene, node: &roxmltree::Node) -> Self
	{
		Self
		{
			odd: scene.texture(Scene::parse_xml_child(node, "OddTexture").parse_xml_attr_str("name")),
			even: scene.texture(Scene::parse_xml_child(node, "EvenTexture").parse_xml_attr_str("name")),
		}
	}*/
}

impl Texture for CheckerTexture
{
	fn value(&self, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
		if sines < 0.0
		{
			self.odd.value(uv, p)
		}
		else
		{
			self.even.value(uv, p)
		}
	}
}
