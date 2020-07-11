use std::{fmt, ops};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec2d {
	pub x: f32,
	pub y: f32,
}

impl Vec2d {
	/// ``(0, 1)``
	pub const UP   : Self = Vec2d { x:  0.0, y:  1.0 };
	/// ``(0, -1)``
	pub const DOWN : Self = Vec2d { x:  0.0, y: -1.0 };
	/// ``(1, 0)``
	pub const RIGHT: Self = Vec2d { x:  1.0, y:  0.0 };
	/// ``(-1, 0)``
	pub const LEFT : Self = Vec2d { x: -1.0, y:  0.0 };
	/// ``(0, 0)``
	pub const ZERO : Self = Vec2d { x:  0.0, y:  0.0 };

	/// Normalizes the vector.
	/// Normalizing a vector very close to ``(0, 0)`` will return the
	/// vector itself.
	#[inline]
	pub fn normalized(self) -> Self {
		if self.x.abs() + self.y.abs() < 0.000000001 {
			return self;
		}
		self * fast_inverse_sqrt(self.mag_sqr())
	}

	/// Takes the dot product of a vector.
	#[inline]
	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y
	}

	/// The magnitude of the vector. 
	/// This is slow because it contains a square root!
	#[inline]
	pub fn mag(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	/// The magnitude of the vector squared.
	/// This is a lot faster than the mag function.
	#[inline]
	pub fn mag_sqr(self) -> f32 {
		self.x * self.x + self.y * self.y
	}
}

impl ops::Div<f32> for Vec2d {
	type Output = Self;

	#[inline]
	fn div(self, other: f32) -> Self::Output {
		Vec2d {
			x: self.x / other,
			y: self.y / other,
		}
	}
}

impl ops::Mul<f32> for Vec2d {
	type Output = Self;

	#[inline]
	fn mul(self, other: f32) -> Self::Output {
		Vec2d {
			x: self.x * other,
			y: self.y * other,
		}
	}
}

impl ops::Add for Vec2d {
	type Output = Self;

	#[inline]
	fn add(self, other: Self) -> Self::Output {
		Vec2d {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl ops::Sub for Vec2d {
	type Output = Self;

	#[inline]
	fn sub(self, other: Self) -> Self::Output {
		Vec2d {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl fmt::Debug for Vec2d {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl fmt::Display for Vec2d {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

/// A fast inverse square root. Originally implemented in
/// ``Quake 3 Arena``. 
///
/// Taken from here:
/// https://en.wikipedia.org/wiki/Fast_inverse_square_root
#[inline]
pub fn fast_inverse_sqrt(number: f32) -> f32 {
	// Comments are also from the original, for comedic effect.
	let mut i;
	let x2; 
	let mut y;
	const THREE_HALFS: f32 = 1.5;

	x2 = number * 0.5;
	y  = number;
	i  = y.to_bits();               // evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );   // what the fuck?
	y  = f32::from_bits(i);
	y  = y * ( THREE_HALFS - ( x2 * y * y ) );   // 1st iteration
	// Use if you want more accuracy. But we want speed!!!
	// y  = y * ( THREE_HALFS - ( x2 * y * y ) );   // 2nd iteration, this can be removed
	y
}
