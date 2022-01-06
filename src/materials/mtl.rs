use crate::hittable::HitRecord;
use crate::math::vec::
{
	Vec2d,
	Vec3d,
};
use crate::ray::Ray;
use crate::textures::
{
	Texture,
	image_texture::ImageTexture,
};

use rand::Rng;

use std::sync::Arc;

use super::Material;

use wavefront_obj::mtl;

enum MtlIllumination
{
	Ambient,
	AmbientDiffuse,
	AmbientDiffuseSpecular,
	ReflectionRayTrace,
}

impl From<mtl::Illumination> for MtlIllumination
{
	fn from(illumination: mtl::Illumination) -> Self
	{
		match illumination
		{
			mtl::Illumination::Ambient                => MtlIllumination::Ambient,
			mtl::Illumination::AmbientDiffuse         => MtlIllumination::AmbientDiffuse,
			mtl::Illumination::AmbientDiffuseSpecular => MtlIllumination::AmbientDiffuseSpecular,
			mtl::Illumination::ReflectionRayTrace     => MtlIllumination::ReflectionRayTrace,
		}
	}
}

pub struct Mtl
{
	specular_coefficient: f64,
	color_ambient: Vec3d,
	color_diffuse: Vec3d,
	color_specular: Vec3d,
	color_emissive: Option<Vec3d>,
	optical_density: Option<f64>,
	alpha: f64,
	illumination: MtlIllumination,
	ambient_map: Option<Arc::<dyn Texture>>,
	diffuse_map: Option<Arc::<dyn Texture>>,
	specular_map: Option<Arc::<dyn Texture>>,
	specular_exponent_map: Option<Arc::<dyn Texture>>,
	dissolve_map: Option<Arc::<dyn Texture>>,
	displacement_map: Option<Arc::<dyn Texture>>,
	decal_map: Option<Arc::<dyn Texture>>,
	bump_map: Option<Arc::<dyn Texture>>,
}

impl Mtl
{
	pub fn new(mtl: &mtl::Material) -> Self
	{
		Self{specular_coefficient:  mtl.specular_coefficient,
		     color_ambient:         Self::to_color(mtl.color_ambient),
		     color_diffuse:         Self::to_color(mtl.color_diffuse),
		     color_specular:        Self::to_color(mtl.color_specular),
		     color_emissive:        Self::to_opt_color(mtl.color_emissive),
		     optical_density:       mtl.optical_density,
		     alpha:                 mtl.alpha,
		     illumination:          MtlIllumination::from(mtl.illumination),
		     ambient_map:           Self::to_texture(&mtl.ambient_map),
		     diffuse_map:           Self::to_texture(&mtl.diffuse_map),
		     specular_map:          Self::to_texture(&mtl.specular_map),
		     specular_exponent_map: Self::to_texture(&mtl.specular_exponent_map),
		     dissolve_map:          Self::to_texture(&mtl.dissolve_map),
		     displacement_map:      Self::to_texture(&mtl.displacement_map),
		     decal_map:             Self::to_texture(&mtl.decal_map),
		     bump_map:              Self::to_texture(&mtl.bump_map)}
	}

	fn to_color(col: mtl::Color) -> Vec3d
	{
		Vec3d::new(col.r, col.g, col.b)
	}

	fn to_opt_color(col: Option<mtl::Color>) -> Option<Vec3d>
	{
		match col
		{
			None => None,
			Some(color) => Some(Self::to_color(color))
		}
	}

	fn to_texture(file: &Option<String>) -> Option<Arc::<dyn Texture>>
	{
		match file
		{
			None => None,
			Some(filename) =>
			{
				Some(Arc::new(ImageTexture::new(&filename)))
			}
		}
	}
}

impl Material for Mtl
{
	fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3d, Ray, f64)>
	{
		let mut rng = rand::thread_rng();
		let specular_pos = f64::powf(f64::max(0.0, Vec3d::dot(rec.normal, -r.dir())), self.specular_coefficient);
		if rng.gen_range(0.0..1.0) < specular_pos
		{
			let reflected = Vec3d::reflect(Vec3d::normalize(r.dir()), rec.normal);
			let scattered = Ray::with_time(rec.p, reflected, r.time());
			if Vec3d::dot(scattered.dir(), rec.normal) > 0.0
			{
				let mut color_specular = self.color_specular;
				if let Some(tex) = &self.specular_map
				{
					color_specular += (*tex).value(rec.uv, rec.p);
				}

				return Some((color_specular, scattered, 1.0));
			}

			return None;
		}

		/* diffuse */
		let mut scatter_direction = rec.normal + Vec3d::random_unit_vector();
		if scatter_direction.near_zero()
		{
			scatter_direction = rec.normal;
		}

		let mut color_diffuse = self.color_diffuse;
		if let Some(tex) = &self.diffuse_map
		{
			color_diffuse += tex.value(rec.uv, rec.p);
		}

		Some((color_diffuse, Ray::with_time(rec.p, scatter_direction, r.time()), 1.0))
	}

	fn emitted(&self, _r: &Ray, _rec: &HitRecord, uv: Vec2d, p: Vec3d) -> Vec3d
	{
		match self.color_emissive
		{
			None => Vec3d::zero(),
			Some(color) => color,
		}
	}
}
