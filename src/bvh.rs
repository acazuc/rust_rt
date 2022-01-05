use crate::hittable::{Hittable,HitRecord};
use crate::math::aabb::Aabb;
use crate::ray::Ray;
use crate::scene::Scene;

use rand::Rng;

use std::cmp::Ordering;

use std::sync::Arc;

pub struct BvhNode
{
	left: Option<Arc::<dyn Hittable>>,
	right: Option<Arc::<dyn Hittable>>,
	aabb: Aabb,
}

impl BvhNode
{
	pub fn new(mut objects: Vec<Arc::<dyn Hittable>>, time0: f64, time1: f64) -> Self
	{
		let (left, right) = match objects.len()
		{
			0 => (None, None),
			1 => (Some(objects.remove(0)), None),
			2 => (Some(objects.remove(0)), Some(objects.remove(0))),
			_ => 
			{
				let mut rng = rand::thread_rng();
				let axis = f64::floor(rng.gen_range(0.0..3.0)) as u32;
				let comparator = match axis
				{
					0 => Self::box_x_compare,
					1 => Self::box_y_compare,
					_ => Self::box_z_compare,
				};

				objects.sort_unstable_by(|a, b| comparator(&a, &b));

				let mid = objects.len() / 2;
				let objects_left = objects.drain(0..mid).collect();
				let tmp1: Arc::<dyn Hittable> = Arc::new(BvhNode::new(objects_left, time0, time1));
				let tmp2: Arc::<dyn Hittable> = Arc::new(BvhNode::new(objects, time0, time1));
				(
					Some(tmp1),
					Some(tmp2),
				)
			}
		};

		let aabb = match (&left, &right)
		{
			(None, None) => None,
			(Some(l), None) => l.bounding_box(time0, time1),
			(None, Some(r)) => r.bounding_box(time0, time1),
			(Some(l), Some(r)) =>
			{
				match (l.bounding_box(time0, time1), r.bounding_box(time0, time1))
				{
					(None, None) => None,
					(Some(ll), None) => Some(ll),
					(None, Some(rr)) => Some(rr),
					(Some(ll), Some(rr)) => Some(Aabb::surrounding_box(&ll, &rr)),
				}
			}
		};

		let aabb = match aabb
		{
			None => panic!("No bounding box!"),
			Some(aabb) => aabb,
		};

		Self{left, right, aabb}
	}

	fn box_compare(a: &Arc::<dyn Hittable>, b: &Arc::<dyn Hittable>, axis: usize) -> Ordering
	{
		match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0))
		{
			(None, None) => panic!("no bounding box for compare"),
			(Some(_l), None) => panic!("no bounding box for compare"),
			(None, Some(_r)) => panic!("no bounding box for compare"),
			(Some(l), Some(r)) =>
			{
				if l.min().v[axis] < r.min().v[axis]
				{
					Ordering::Less
				}
				else
				{
					Ordering::Greater
				}
			}
		}
	}

	fn box_x_compare(a: &Arc::<dyn Hittable>, b: &Arc::<dyn Hittable>) -> Ordering
	{
		Self::box_compare(a, b, 0)
	}

	fn box_y_compare(a: &Arc::<dyn Hittable>, b: &Arc::<dyn Hittable>) -> Ordering
	{
		Self::box_compare(a, b, 1)
	}

	fn box_z_compare(a: &Arc::<dyn Hittable>, b: &Arc::<dyn Hittable>) -> Ordering
	{
		Self::box_compare(a, b, 2)
	}
}

impl Hittable for BvhNode
{
	fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>
	{
		if !self.aabb.hit(ray, tmin, tmax)
		{
			return None;
		}

		match (&self.left, &self.right)
		{
			(None, None) => None,
			(Some(l), None) => l.hit(ray, tmin, tmax),
			(None, Some(r)) => r.hit(ray, tmin, tmax),
			(Some(l), Some(r)) => match (l.hit(ray, tmin, tmax), r.hit(ray, tmin, tmax))
			{
				(None, None) => None,
				(Some(ll), None) => Some(ll),
				(None, Some(rr)) => Some(rr),
				(Some(ll), Some(rr)) =>
				{
					if ll.t < rr.t
					{
						Some(ll)
					}
					else
					{
						Some(rr)
					}
				}
			}
		}
	}

	fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb>
	{
		Some(self.aabb)
	}

	fn bvh_depth(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(f64, u32)>
	{
		if !self.aabb.hit(ray, tmin, tmax)
		{
			return None;
		}

		return match (&self.left, &self.right)
		{
			(None, None) => None,
			(Some(l), None) =>
			{
				if let Some(ll) = l.bvh_depth(ray, tmin, tmax)
				{
					Some((ll.0, ll.1 + 1))
				}
				else
				{
					None
				}
			}
			(None, Some(r)) =>
			{
				if let Some(rr) = r.bvh_depth(ray, tmin, tmax)
				{
					Some((rr.0, rr.1 + 1))
				}
				else
				{
					None
				}
			}
			(Some(l), Some(r)) => match (l.bvh_depth(ray, tmin, tmax), r.bvh_depth(ray, tmin, tmax))
			{
				(None, None) => None,
				(Some(ll), None) => Some((ll.0, ll.1 + 1)),
				(None, Some(rr)) => Some((rr.0, rr.1 + 1)),
				(Some(ll), Some(rr)) =>
				{
					if ll.0 < rr.0
					{
						Some((ll.0, ll.1 + 1))
					}
					else
					{
						Some((rr.0, rr.1 + 1))
					}
				},
			}
		}
	}
}
