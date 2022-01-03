#![allow(dead_code)]

use std::default::Default;

use std::ops::
{
	Add,
	AddAssign,
	Div,
	DivAssign,
	Neg,
	Mul,
	MulAssign,
	Rem,
	RemAssign,
	Sub,
	SubAssign,
};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct VecN<T, const N: usize>
{
	pub v: [T; N],
}

impl<T, const N: usize> Default for VecN<T, N>
where T: Default + Copy
{
	fn default() -> Self
	{
		let v: [T; N] = [T::default(); N];
		Self{v}
	}
}

impl<T, const N: usize> VecN<T, N>
where T: Mul<Output = T> + AddAssign + Default + Copy
{
	pub fn dot(lhs: Self, rhs: Self) -> T
	{
		let mut ret: T = T::default();
		for i in 0..N
		{
			ret += lhs.v[i] * rhs.v[i];
		}
		ret
	}

	pub fn for_each(&self, func: &dyn Fn(T) -> T) -> Self
	{
		let mut ret = *self;
		for x in &mut ret.v
		{
			*x = func(*x);
		}
		ret
	}
}

type Vec4<T> = VecN<T, 4>;
type Vec3<T> = VecN<T, 3>;
type Vec2<T> = VecN<T, 2>;
type Vec1<T> = VecN<T, 1>;

pub(crate) type Vec4b = Vec4<u8>;
pub(crate) type Vec4s = Vec4<i16>;
pub(crate) type Vec4i = Vec4<i32>;
pub(crate) type Vec4u = Vec4<u32>;
pub(crate) type Vec4f = Vec4<f32>;
pub(crate) type Vec4d = Vec4<f64>;

pub(crate) type Vec3b = Vec3<u8>;
pub(crate) type Vec3s = Vec3<i16>;
pub(crate) type Vec3i = Vec3<i32>;
pub(crate) type Vec3u = Vec3<u32>;
pub(crate) type Vec3f = Vec3<f32>;
pub(crate) type Vec3d = Vec3<f64>;

pub(crate) type Vec2b = Vec2<u8>;
pub(crate) type Vec2s = Vec2<i16>;
pub(crate) type Vec2i = Vec2<i32>;
pub(crate) type Vec2u = Vec2<u32>;
pub(crate) type Vec2f = Vec2<f32>;
pub(crate) type Vec2d = Vec2<f64>;

impl Vec3d
{
	pub fn length(lhs: Self) -> f64
	{
		f64::sqrt(Self::dot(lhs, lhs))
	}

	pub fn normalize(lhs: Self) -> Self
	{
		lhs / Self::length(lhs)
	}

	pub fn random(min: f64, max: f64) -> Self
	{
		let mut rng = rand::thread_rng();
		Self{v: [rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max)]}
	}

	pub fn random_in_unit_sphere() -> Self
	{
		loop
		{
			let v = Self::random(-1.0, 1.0);
			if Self::dot(v, v) >= 1.0
			{
				continue
			}

			return v;
		}
	}

	pub fn random_unit_vector() -> Self
	{
		Self::normalize(Self::random_in_unit_sphere())
	}

	pub fn random_in_unit_disk() -> Self
	{
		let mut rng = rand::thread_rng();
		loop
		{
			let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
			if Self::dot(p, p) < 1.0
			{
				return p;
			}
		}
	}

	pub fn random_in_hemisphere(normal: Self) -> Self
	{
		let ret = Self::random_in_unit_sphere();
		if Self::dot(ret, normal) > 0.0
		{
			return ret;
		}

		-ret
	}

	pub fn near_zero(self) -> bool
	{
		self.v.iter().any(|a| f64::abs(*a) < f64::EPSILON)
	}

	pub fn reflect(lhs: Self, rhs: Self) -> Self
	{
		lhs - rhs * Self::dot(lhs, rhs) * 2.0
	}

	pub fn refract(lhs: Self, n: Self, etai_over_etat: f64) -> Self
	{
		let cos_theta = f64::min(Self::dot(n, -lhs), 1.0);
		let r_out_perp = (lhs + n * cos_theta) * etai_over_etat;
		let r_out_parallel = n * -f64::sqrt(f64::abs(1.0 - Self::dot(r_out_perp, r_out_perp)));
		r_out_perp + r_out_parallel
	}

	pub fn zero() -> Self
	{
		Self::newv(0.0)
	}

	pub fn one() -> Self
	{
		Self::newv(1.0)
	}
}

impl From<[f32; 3]> for Vec3d
{
	fn from(data: [f32; 3]) -> Self
	{
		Self::new(data[0] as f64, data[1] as f64, data[2] as f64)
	}
}

impl<T: Copy> Vec4<T>
{
	pub fn new(x: T, y: T, z: T, w: T) -> Self
	{
		Self{v: [x, y, z, w]}
	}

	pub fn newv(v: T) -> Self
	{
		Self{v: [v, v, v, v]}
	}

	pub fn x(self) -> T
	{
		self.v[0]
	}

	pub fn y(self) -> T
	{
		self.v[1]
	}

	pub fn z(self) -> T
	{
		self.v[2]
	}

	pub fn w(self) -> T
	{
		self.v[3]
	}
}

impl<T: Copy> Vec3<T>
{
	pub fn new(x: T, y: T, z: T) -> Self
	{
		Self{v: [x, y, z]}
	}

	pub fn newv(v: T) -> Self
	{
		Self{v: [v, v, v]}
	}

	pub fn x(self) -> T
	{
		self.v[0]
	}

	pub fn y(self) -> T
	{
		self.v[1]
	}

	pub fn z(self) -> T
	{
		self.v[2]
	}
}

impl<T> Vec3<T>
where T: Copy + Mul<Output = T> + Sub<Output = T>
{
	pub fn cross(lhs: Self, rhs: Self) -> Self
	{
		Self::new(lhs.y() * rhs.z() - lhs.z() * rhs.y(),
		          lhs.z() * rhs.x() - lhs.x() * rhs.z(),
		          lhs.x() * rhs.y() - lhs.y() * rhs.x())
	}
}

impl<T: Copy> Vec2<T>
{
	pub fn new(x: T, y: T) -> Self
	{
		Self{v: [x, y]}
	}

	pub fn newv(v: T) -> Self
	{
		Self{v: [v, v]}
	}

	pub fn x(self) -> T
	{
		self.v[0]
	}

	pub fn y(self) -> T
	{
		self.v[1]
	}
}

impl<T: Copy> Vec1<T>
{
	pub fn new(x: T) -> Self
	{
		Self{v: [x]}
	}

	pub fn newv(v: T) -> Self
	{
		Self{v: [v]}
	}

	pub fn x(self) -> T
	{
		self.v[0]
	}
}

impl<T: Copy> From<(T, T, T, T)> for Vec4<T>
{
	fn from(data: (T, T, T, T)) -> Self
	{
		Vec4::new(data.0, data.1, data.2, data.3)
	}
}

impl<T: Copy> From<[T; 4]> for Vec4<T>
{
	fn from(data: [T; 4]) -> Self
	{
		Vec4::new(data[0], data[1], data[2], data[3])
	}
}

impl<T: Copy> From<(T, T, T)> for Vec3<T>
{
	fn from(data: (T, T, T)) -> Self
	{
		Vec3::new(data.0, data.1, data.2)
	}
}

impl<T: Copy> From<[T; 3]> for Vec3<T>
{
	fn from(data: [T; 3]) -> Self
	{
		Vec3::new(data[0], data[1], data[2])
	}
}

impl<T: Copy> From<(T, T)> for Vec2<T>
{
	fn from(data: (T, T)) -> Self
	{
		Vec2::new(data.0, data.1)
	}
}

impl<T: Copy> From<[T; 2]> for Vec2<T>
{
	fn from(data: [T; 2]) -> Self
	{
		Vec2::new(data[0], data[1])
	}
}

impl<T: Copy> From<T> for Vec1<T>
{
	fn from(data: T) -> Self
	{
		Vec1::new(data)
	}
}

impl<T: Copy> From<[T; 1]> for Vec1<T>
{
	fn from(data: [T; 1]) -> Self
	{
		Vec1::new(data[0])
	}
}

impl<T, const N: usize> std::cmp::PartialEq<Self> for VecN<T, N>
where T: std::cmp::PartialEq
{
	fn eq(&self, rhs: &Self) -> bool
	{
		self.v.iter().zip(&rhs.v).all(|(a, b)| a.eq(&b))
	}
}

macro_rules! create_overload
{
	($op:ident, $fn:ident) =>
	{
		impl<T, const N: usize> $op<Self> for VecN<T, N>
		where T: $op<Output = T> + Default + Copy
		{
			type Output = Self;

			fn $fn(self, rhs: Self) -> Self
			{
				let mut v = Self::default();
				for i in 0..N
				{
					v.v[i] = $op::$fn(self.v[i], rhs.v[i]);
				}
				v
			}
		}

		impl<T, const N: usize> $op<T> for VecN<T, N>
		where T: $op<Output = T> + Default + Copy
		{
			type Output = Self;

			fn $fn(self, rhs: T) -> Self
			{
				let mut v = Self::default();
				for i in 0..N
				{
					v.v[i] = $op::$fn(self.v[i], rhs);
				}
				v
			}
		}
	};
}

macro_rules! create_overload_assign
{
	($op:ident, $fn:ident) =>
	{
		impl<T, const N: usize> $op<Self> for VecN<T, N>
		where T: $op + Default + Copy
		{
			fn $fn(&mut self, rhs: Self)
			{
				for i in 0..N
				{
					$op::$fn(&mut self.v[i], rhs.v[i]);
				}
			}
		}

		impl<T, const N: usize> $op<T> for VecN<T, N>
		where T: $op + Default + Copy
		{
			fn $fn(&mut self, rhs: T)
			{
				for i in 0..N
				{
					$op::$fn(&mut self.v[i], rhs);
				}
			}
		}
	};
}

impl<T, const N: usize> Neg for VecN<T, N>
where T: Neg<Output = T> + Default + Copy
{
	type Output = Self;

	fn neg(self) -> Self
	{
		let mut v = Self::default();
		for i in 0..N
		{
			v.v[i] = Neg::neg(self.v[i]);
		}
		v
	}
}

create_overload!(Add, add);
create_overload_assign!(AddAssign, add_assign);
create_overload!(Sub, sub);
create_overload_assign!(SubAssign, sub_assign);
create_overload!(Mul, mul);
create_overload_assign!(MulAssign, mul_assign);
create_overload!(Div, div);
create_overload_assign!(DivAssign, div_assign);
create_overload!(Rem, rem);
create_overload_assign!(RemAssign, rem_assign);

#[cfg(test)]
mod tests
{

	use super::*;

	#[test]
	fn test_zero()
	{
		let v = Vec4f::default();
		assert_eq!((v.v[0], v.v[1], v.v[2], v.v[3]), (f32::default(), f32::default(), f32::default(), f32::default()));
	}

	#[test]
	fn test_eq()
	{
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		assert_eq!(v1, v2);
	}

	#[test]
	fn test_ne()
	{
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(2.0, 2.0, 3.0, 4.0);
		assert_ne!(v1, v2);
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(1.0, 3.0, 3.0, 4.0);
		assert_ne!(v1, v2);
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(1.0, 2.0, 4.0, 4.0);
		assert_ne!(v1, v2);
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(1.0, 2.0, 3.0, 5.0);
		assert_ne!(v1, v2);
	}

	#[test]
	fn test_add()
	{
		let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
		let v2 = Vec4f::new(5.0, 6.0, 7.0, 8.0);
		let v = v1 + v2;
		assert_eq!((v.v[0], v.v[1], v.v[2], v.v[3]), (6.0, 8.0, 10.0, 12.0));
	}

}
