use crate::bvh::BvhNode;
use crate::hittable::
{
	HitRecord,
	Hittable,
};
use crate::math::
{
	aabb::Aabb,
	vec::
	{
		Vec2d,
		Vec3d,
	}
};
use crate::materials::
{
	Material,
	lambertian::Lambertian,
};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::textures::
{
	Texture,
	solid_color::SolidColor,
};

use std::sync::Arc;

use super::triangle::Triangle;

use wavefront_obj::obj;

pub struct Obj
{
	bvh: BvhNode,
}

impl Obj
{
	pub fn new(filename: &str, origin: Vec3d, scale: Vec3d) -> Self
	{
		let mat: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(1.0, 1.0, 1.0)))));
		//let mat: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
		let content = std::fs::read_to_string(filename).expect("can't read file");
		let res = obj::parse(content);
		if let Result::Ok(wf) = &res
		{
			let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();
			for obj in &wf.objects
			{
				let mut geometries: Vec::<Arc::<dyn Hittable>> = Vec::new();
				for geometry in &obj.geometry
				{
					let mut shapes: Vec::<Arc::<dyn Hittable>> = Vec::new();
					for shape in &geometry.shapes
					{
						match shape.primitive
						{
							wavefront_obj::obj::Primitive::Triangle(k, i, j) =>
							{
								shapes.push(Arc::new(Triangle::new
								(
									Self::get_vertices(&obj, &i, &j, &k, origin, scale),
									Self::get_tex_vertices(&obj, &i, &j, &k),
									Self::get_norm_vertices(&obj, &i, &j, &k),
									mat.clone()
								)));
							},
							_ =>
							{
								println!("unsupported primitive");
							}
						}
					}
					if !shapes.is_empty()
					{
						geometries.push(Arc::new(BvhNode::new(shapes, 0.0, 0.0)));
					}
				}
				if !geometries.is_empty()
				{
					objects.push(Arc::new(BvhNode::new(geometries, 0.0, 0.0)));
				}
			}

			let bvh = BvhNode::new(objects, 0.0, 0.0);
			return Self{bvh};
		}

		panic!("failed to parse {}: {:?}", filename, res);
	}

	fn get_vertices(obj: &obj::Object, i: &obj::VTNIndex, j: &obj::VTNIndex, k: &obj::VTNIndex, origin: Vec3d, scale: Vec3d) -> [Vec3d; 3]
	{
		let vi = obj.vertices[i.0];
		let vj = obj.vertices[j.0];
		let vk = obj.vertices[k.0];
		[
			Vec3d::new(vi.x, vi.y, vi.z) * scale + origin,
			Vec3d::new(vj.x, vj.y, vj.z) * scale + origin,
			Vec3d::new(vk.x, vk.y, vk.z) * scale + origin
		]
	}

	fn get_tex_vertice(obj: &obj::Object, n: &obj::VTNIndex) -> Vec2d
	{
		if let Some(idx) = n.1
		{
			return Vec2d::new(obj.tex_vertices[idx].u, obj.tex_vertices[idx].v);
		}

		Vec2d::new(0.0, 0.0)
	}

	fn get_tex_vertices(obj: &obj::Object, i: &obj::VTNIndex, j: &obj::VTNIndex, k: &obj::VTNIndex) -> [Vec2d; 3]
	{
		[
			Self::get_tex_vertice(obj, i),
			Self::get_tex_vertice(obj, j),
			Self::get_tex_vertice(obj, k),
		]
	}

	fn get_norm_vertice(obj: &obj::Object, n: &obj::VTNIndex) -> Vec3d
	{
		if let Some(idx) = n.2
		{
			return Vec3d::new(obj.normals[idx].x, obj.normals[idx].y, obj.normals[idx].z);
		}

		Vec3d::new(0.0, 0.0, 0.0)
	}

	fn get_norm_vertices(obj: &obj::Object, i: &obj::VTNIndex, j: &obj::VTNIndex, k: &obj::VTNIndex) -> [Vec3d; 3]
	{
		[
			Self::get_norm_vertice(obj, i),
			Self::get_norm_vertice(obj, j),
			Self::get_norm_vertice(obj, k),
		]
	}
}

impl Hittable for Obj
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		self.bvh.hit(r, tmin, tmax)
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		self.bvh.bounding_box(time0, time1)
	}

	fn bvh_depth(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		self.bvh.bvh_depth(r, tmin, tmax)
	}
}
