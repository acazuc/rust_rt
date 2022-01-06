use crate::math::vec::Vec3d;

pub struct Onb
{
	axis: [Vec3d; 3],
}

impl Onb
{
	pub fn from_w(n: Vec3d) -> Self
	{
		let aw = Vec3d::normalize(n);
		let a;
		if f64::abs(aw.x()) > 0.9
		{
			a = Vec3d::new(0.0, 1.0, 0.0);
		}
		else
		{
			a = Vec3d::new(1.0, 0.0, 0.0);
		}
		let av = Vec3d::normalize(Vec3d::cross(aw, a));
		let au = Vec3d::cross(aw, av);
		Self{axis: [au, av, aw]}
	}

	pub fn local(&self, a: Vec3d) -> Vec3d
	{
		self.axis[0] * a.x() + self.axis[1] * a.y() + self.axis[2] * a.z()
	}

	pub fn u(&self) -> Vec3d
	{
		self.axis[0]
	}

	pub fn v(&self) -> Vec3d
	{
		self.axis[1]
	}

	pub fn w(&self) -> Vec3d
	{
		self.axis[2]
	}
}
