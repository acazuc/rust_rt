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
use crate::materials::Material;
use crate::ray::Ray;

use std::sync::Arc;

use super::triangle::Triangle;

pub struct Stl
{
	bvh: BvhNode,
}

impl Stl
{
	pub fn new(filename: &str, origin: Vec3d, scale: Vec3d, mat: Arc::<dyn Material>) -> Self
	{
		if let Result::Ok(file) = std::fs::File::open(filename)
		{
			let mut reader = std::io::BufReader::new(&file);
			if let Result::Ok(mesh) = nom_stl::parse_stl(&mut reader)
			{
				let mut triangles: Vec::<Arc::<dyn Hittable>> = Vec::new();
				for triangle in mesh.triangles()
				{
					triangles.push(Arc::new(Triangle::new
					(
						[Vec3d::from(triangle.vertices()[0]) * scale + origin, Vec3d::from(triangle.vertices()[1]) * scale + origin, Vec3d::from(triangle.vertices()[2]) * scale + origin],
						[Vec2d::new(0.0, 0.0), Vec2d::new(0.0, 0.0), Vec2d::new(0.0, 0.0)],
						[Vec3d::from(triangle.normal()), Vec3d::from(triangle.normal()), Vec3d::from(triangle.normal())],
						mat.clone()
					)));
				}

				let bvh = BvhNode::new(triangles, 0.0, 0.0);
				return Self{bvh};
			}

			panic!("can't parse file {}", filename);
		}

		panic!("can't open file {}", filename);
	}
}

impl Hittable for Stl
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		self.bvh.hit(r, tmin, tmax)
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		self.bvh.bounding_box(time0, time1)
	}
}
