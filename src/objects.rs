use crate::hittable::Hittable;
use crate::hittable::HitRecord;

use crate::math::{Vec3d,Vec2d};

use crate::ray::Ray;

use crate::materials::{Material,Lambertian};

use crate::textures::SolidColor;

use std::sync::Arc;

use crate::aabb::Aabb;

use crate::bvh::BvhNode;

use wavefront_obj::obj;

pub struct Sphere
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Sphere
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Sphere{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Sphere
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center;
		let a = Vec3d::dot(r.dir(), r.dir());
		let half_b = Vec3d::dot(oc, r.dir());
		let c = Vec3d::dot(oc, oc) - self.radius * self.radius;
		let d = half_b * half_b - a * c;
		if d < 0.0
		{
			return None;
		}

		let sqrt_d = f64::sqrt(d);
		let mut t = (-half_b - sqrt_d) / a;
		if t < tmin || t > tmax
		{
			t = (-half_b + sqrt_d) / a;
			if t < tmin || t > tmax
			{
				return None;
			}
		}

		let p = r.at(t);
		let normal = (p - self.center) / self.radius;
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}

pub struct MovingSphere
{
	center0: Vec3d,
	center1: Vec3d,
	time0: f64,
	time1: f64,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl MovingSphere
{
	pub fn new(center0: Vec3d, center1: Vec3d, time0: f64, time1: f64, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		MovingSphere{center0, center1, time0, time1, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(-p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}

	fn center(&self, time: f64) -> Vec3d
	{
		self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
	}
}

impl Hittable for MovingSphere
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center(r.time());
		let a = Vec3d::dot(r.dir(), r.dir());
		let half_b = Vec3d::dot(oc, r.dir());
		let c = Vec3d::dot(oc, oc) - self.radius * self.radius;
		let d = half_b * half_b - a * c;
		if d < 0.0
		{
			return None;
		}

		let sqrt_d = f64::sqrt(d);
		let mut t = (-half_b - sqrt_d) / a;
		if t < tmin || t > tmax
		{
			t = (-half_b + sqrt_d) / a;
			if t < tmin || t > tmax
			{
				return None;
			}
		}

		let p = r.at(t);
		let normal = (p - self.center(r.time())) / self.radius;
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>
	{
		let box0 = Aabb::new(self.center(time0) - Vec3d::newv(self.radius),
		                     self.center(time0) + Vec3d::newv(self.radius));
		let box1 = Aabb::new(self.center(time1) - Vec3d::newv(self.radius),
		                     self.center(time1) + Vec3d::newv(self.radius));
		Some(Aabb::surrounding_box(&box0, &box1))
	}
}

pub struct Cylinder
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Cylinder
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Cylinder{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(0.0);
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Cylinder
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let mut oc = r.orig() - self.center;
		oc = Vec3d::new(oc.x(), 0.0, oc.z());
		let mut rdir = r.dir();
		rdir = Vec3d::new(rdir.x(), 0.0, rdir.z());
		let a = Vec3d::dot(rdir, rdir);
		let half_b = Vec3d::dot(oc, rdir);
		let c = Vec3d::dot(oc, oc) - self.radius * self.radius;
		let d = half_b * half_b - a * c;
		if d < 0.0
		{
			return None;
		}

		let sqrt_d = f64::sqrt(d);
		let mut t = (-half_b - sqrt_d) / a;
		if t < tmin || t > tmax
		{
			t = (-half_b + sqrt_d) / a;
			if t < tmin || t > tmax
			{
				return None;
			}
		}

		let p = r.at(t);
		let mut normal = (p - self.center) / self.radius;
		normal = Vec3d::new(normal.x(), 0.0, normal.z());
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}

pub struct Cone
{
	center: Vec3d,
	radius : f64,
	material: Arc::<dyn Material>,
}

impl Cone
{
	pub fn new(center: Vec3d, radius: f64, material: Arc::<dyn Material>) -> Self
	{
		Cone{center, radius, material}
	}

	pub fn get_uv(&self, p: &Vec3d) -> Vec2d
	{
		let theta = f64::acos(p.y());
		let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
		Vec2d::new(phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
	}
}

impl Hittable for Cone
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let oc = r.orig() - self.center;
		let oc2 = Vec3d::new(oc.x(), -oc.y(), oc.z());
		let rdir = r.dir();
		let rdir2 = Vec3d::new(rdir.x(), -rdir.y(), rdir.z());
		let a = Vec3d::dot(rdir2, rdir);
		let half_b = Vec3d::dot(oc, rdir2);
		let c = Vec3d::dot(oc2, oc) - self.radius * self.radius;
		let d = half_b * half_b - a * c;
		if d < 0.0
		{
			return None;
		}

		let sqrt_d = f64::sqrt(d);
		let mut t = (-half_b - sqrt_d) / a;
		if t < tmin || t > tmax
		{
			t = (-half_b + sqrt_d) / a;
			if t < tmin || t > tmax
			{
				return None;
			}
		}

		let p = r.at(t);
		let mut normal = (p - self.center) / self.radius;
		normal = Vec3d::new(normal.x(), -normal.y(), normal.z());
		Some(HitRecord::new(r, p, t, self.get_uv(&normal), normal, self.material.clone()))
	}

	fn bounding_box(&self, _time: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new(self.center - Vec3d::newv(self.radius), self.center + Vec3d::newv(self.radius)))
	}
}

pub struct Triangle
{
	p: [Vec3d; 3],
	e: [Vec3d; 2],
	uv: [Vec2d; 3],
	norm: [Vec3d; 3],
	material: Arc::<dyn Material>,
}

impl Triangle
{
	pub fn new(p: [Vec3d; 3], uv: [Vec2d; 3], norm: [Vec3d; 3], material: Arc::<dyn Material>) -> Self
	{
		let e = [p[1] - p[0], p[2] - p[0]];
		let ret = Self{p, e, uv, norm, material};
		ret
	}

	fn get_uv(&self, u: f64, v: f64) -> Vec2d
	{
		self.uv[1] * u + self.uv[2] * v + self.uv[0] * (1.0 - u - v)
	}

	fn get_norm(&self, u: f64, v: f64) -> Vec3d
	{
		self.norm[1] * u + self.norm[2] * v + self.norm[0] * (1.0 - u - v)
	}
}

impl Hittable for Triangle
{
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		let p = Vec3d::cross(r.dir(), self.e[1]);
		let mut det = Vec3d::dot(self.e[0], p);
		if det > -std::f64::EPSILON && det < std::f64::EPSILON
		{
			return None;
		}

		det = 1.0 / det;
		let tt = r.orig() - self.p[0];
		let u = Vec3d::dot(tt, p) * det;
		if u < std::f64::EPSILON || u > 1.0 + std::f64::EPSILON
		{
			return None;
		}

		let q = Vec3d::cross(tt, self.e[0]);
		let v = Vec3d::dot(r.dir(), q) * det;
		if v < std::f64::EPSILON || u + v > 1.0 + std::f64::EPSILON
		{
			return None;
		}

		let t = Vec3d::dot(self.e[1], q) * det;
		if t < tmin || t > tmax
		{
			return None;
		}

		let p = r.at(t);
		Some(HitRecord::new(r, p, t, self.get_uv(u, v), self.get_norm(u, v), self.material.clone()))
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(Aabb::new
		(
			Vec3d::new
			(
				f64::min(self.p[0].x(), f64::min(self.p[1].x(), self.p[2].x())),
				f64::min(self.p[0].y(), f64::min(self.p[1].y(), self.p[2].y())),
				f64::min(self.p[0].z(), f64::min(self.p[1].z(), self.p[2].z()))
			),
			Vec3d::new
			(
				f64::max(self.p[0].x(), f64::max(self.p[1].x(), self.p[2].x())),
				f64::max(self.p[0].y(), f64::max(self.p[1].y(), self.p[2].y())),
				f64::max(self.p[0].z(), f64::max(self.p[1].z(), self.p[2].z()))
			)
		))
	}
}

pub struct Wavefront
{
	bvh: BvhNode,
}

impl Wavefront
{
	pub fn new(filename: &str, origin: Vec3d, scale: Vec3d) -> Self
	{
		let mat: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(1.0, 0.8, 0.8)))));
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

impl Hittable for Wavefront
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
