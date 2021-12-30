use crate::math::Vec3d;

use crate::ray::Ray;

use rand::Rng;

pub struct Camera
{
	origin: Vec3d,
	lower_left_corner: Vec3d,
	horizontal: Vec3d,
	vertical: Vec3d,
	u: Vec3d,
	v: Vec3d,
	w: Vec3d,
	lens_radius: f64,
	time0: f64,
	time1: f64,
}

impl Camera
{
	pub fn new(lookfrom: Vec3d, lookat: Vec3d, vup: Vec3d, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self
	{
		Camera::with_time(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, focus_dist, 0.0, 0.0)
	}

	pub fn with_time(lookfrom: Vec3d, lookat: Vec3d, vup: Vec3d, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> Self
	{
		let theta = f64::to_radians(vfov);
		let h = f64::tan(theta / 2.0);
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = Vec3d::unit_vector(lookfrom - lookat);
		let u = Vec3d::unit_vector(Vec3d::cross(vup, w));
		let v = Vec3d::cross(w, u);

		let horizontal = u * viewport_width * focus_dist;
		let vertical = v * viewport_height * focus_dist;
		let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
		let lens_radius = aperture / 2.0;

		Camera{origin: lookfrom, lower_left_corner, horizontal, vertical, u, v, w, lens_radius, time0, time1}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray
	{
		let rd = Vec3d::random_in_unit_disk() * self.lens_radius;
		let offset = self.u * rd.x() + self.v * rd.y();
		if self.time0 != self.time1
		{
			let mut rng = rand::thread_rng();
			return Ray::with_time(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset, rng.gen_range(self.time0..self.time1));
		}

		return Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset);
	}
}
