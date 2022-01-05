use crate::math::vec::Vec3d;

use rand::Rng;

pub struct Perlin
{
	ranvec: Vec::<Vec3d>,
	perm_x: Vec::<i32>,
	perm_y: Vec::<i32>,
	perm_z: Vec::<i32>,
}

impl Perlin
{
	pub fn new() -> Self
	{
		let mut ranvec: Vec::<Vec3d> = Vec::with_capacity(256);
		for _i in 0..256
		{
			ranvec.push(Vec3d::random(-1.0, 1.0));
		}

		let perm_x = Self::generate_perm();
		let perm_y = Self::generate_perm();
		let perm_z = Self::generate_perm();

		Self{ranvec, perm_x, perm_y, perm_z}
	}

	fn generate_perm() -> Vec::<i32>
	{
		let mut data = Vec::<i32>::with_capacity(256);
		for i in 0..256
		{
			data.push(i);
		}

		Self::permute(&mut data);
		data
	}

	fn permute(data: &mut Vec::<i32>)
	{
		let mut rng = rand::thread_rng();
		for i in data.len()-1..0
		{
			let t = rng.gen_range(0..i);
			let tmp = data[i];
			data[i] = data[t];
			data[t] = tmp;
		}
	}

	pub fn noise(&self, p: Vec3d) -> f64
	{
		let u = p.x() - f64::floor(p.x());
		let v = p.y() - f64::floor(p.y());
		let w = p.z() - f64::floor(p.z());

		let i = f64::floor(p.x()) as i32;
		let j = f64::floor(p.y()) as i32;
		let k = f64::floor(p.z()) as i32;

		let mut c = [[[Vec3d::zero(); 2]; 2]; 2];

		for di in 0..2
		{
			for dj in 0..2
			{
				for dk in 0..2
				{
					c[di][dj][dk] = self.ranvec[(
						self.perm_x[((i + di as i32) & 255) as usize] ^
						self.perm_y[((j + dj as i32) & 255) as usize] ^
						self.perm_z[((k + dk as i32) & 255) as usize]) as usize];
				}
			}
		}

		Self::trilinear_interp(&c, u, v, w)
	}

	fn trilinear_interp(c: &[[[Vec3d; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64
	{
		let mut accum = 0.0;

		let uu = u * u * (3.0 - 2.0 * u);
		let vv = v * v * (3.0 - 2.0 * v);
		let ww = w * w * (3.0 - 2.0 * w);

		for i in 0..2
		{
			for j in 0..2
			{
				for k in 0..2
				{
					let weight_v = Vec3d::new(u - i as f64, v - j as f64, w - k as f64);
					accum += (uu * i as f64 + (1 - i) as f64 * (1.0 - uu)) *
					         (vv * j as f64 + (1 - j) as f64 * (1.0 - vv)) *
					         (ww * k as f64 + (1 - k) as f64 * (1.0 - ww)) *
					         Vec3d::dot(c[i][j][k], weight_v);
				}
			}
		}

		accum
	}

	pub fn turb(&self, p: Vec3d, depth: usize) -> f64
	{
		let mut accum = 0.0;
		let mut temp_p = p;
		let mut weight = 1.0;

		for _i in 0..depth
		{
			accum += weight * self.noise(temp_p);
			weight *= 0.5;
			temp_p *= 2.0;
		}

		f64::abs(accum)
	}
}
