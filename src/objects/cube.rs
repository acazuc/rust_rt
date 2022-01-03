use crate::bvh::BvhNode;
use crate::hittable::
{
	HitRecord,
	Hittable,
};
use crate::math::
{
	aabb::Aabb,
	vec::Vec3d,
};
use crate::materials::Material;
use crate::ray::Ray;

use std::sync::Arc;

use super::rect::
{
	XYRect,
	XZRect,
	YZRect,
};

pub struct Cube
{
	p0: Vec3d,
	p1: Vec3d,
	bvh: BvhNode,
}

impl Cube
{
	pub fn new(p0: Vec3d, p1: Vec3d, material: Arc::<dyn Material>) -> Self
	{
		let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

		objects.push(Arc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone())));
		objects.push(Arc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone())));

		objects.push(Arc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone())));
		objects.push(Arc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone())));

		objects.push(Arc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone())));
		objects.push(Arc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone())));

		let bvh = BvhNode::new(objects, 0.0, 0.0);
		Self{p0, p1, bvh}
	}
}

impl Hittable for Cube
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
