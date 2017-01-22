use prelude::*;
use maths::{Vec3, VecType, Angle};
use glium::uniforms::{AsUniformValue, UniformValue};

/// A 2-dimensional vector.
#[derive(Copy, Clone)]
pub struct Vec2<T: Debug + Float = f32>(pub T, pub T);

impl<T> Vec2<T> where T: Debug + Float {
    /// Creates a new instances.
    pub fn new() -> Vec2<T> {
        Vec2::<T>(T::zero(), T::zero())
    }
    /// Returns the length of the vector
    pub fn len(self: &Self) -> T {
        (self.0*self.0 + self.1*self.1).sqrt()
    }
    /// Returns the direction of the vector in radians.
    pub fn to_radians(self: &Self) -> T {
        self.1.atan2(self.0)
    }
    /// Returns the direction of the vector in degrees.
    pub fn to_degrees(self: &Self) -> T {
        self.to_radians().to_degrees()
    }
    /// Returns the direction of the vector as an angle instance.
    pub fn to_angle(self: &Self) -> Angle<T> {
        Angle(self.to_radians())
    }
    /// Creates a unit-vector from the angle given in radians.
    pub fn from_radians(radians: T) -> Vec2<T> {
        Vec2::<T>(radians.cos(), radians.sin())
    }
    /// Creates a unit-vector from the angle given in degrees.
    pub fn from_degrees(degrees: T) -> Vec2<T> {
        Self::from_radians(degrees.to_radians())
    }
    /// Creates a unit-vector from given angle.
    pub fn from_angle(angle: Angle<T>) -> Vec2<T> {
        Self::from_radians(angle.to_radians())
    }
    /// Normalizes the vector.
    pub fn normalize(mut self: Self) -> Self {
        let len = self.len();
        self.0 = self.0 / len;
        self.1 = self.1 / len;
        self
    }
    /// Extends the vector by given length.
    pub fn extend(mut self: Self, extension_len: T) -> Self {
        let base_len = self.len();
        let new_len = base_len + extension_len;
        let factor = new_len / base_len;
        self.0 = self.0 * factor;
        self.1 = self.1 * factor;
        self
    }
}

impl<T> VecType<T> for Vec2<T> where T: Debug + Float {
    fn as_vec3(&self, neutral: T) -> Vec3<T> {
        Vec3::<T>(self.0, self.1, neutral)
    }
}

impl<T> Neg for Vec2<T> where T: Debug + Float {
    type Output = Vec2<T>;

    fn neg(self) -> Vec2<T> {
        Vec2::<T>(-self.0, -self.1)
    }
}

impl<T> Add for Vec2<T> where T: Debug + Float {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::<T>(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> AddAssign for Vec2<T> where T: Debug + Float {
    fn add_assign(self: &mut Self, other: Vec2<T>) {
        *self = Vec2::<T> (
            self.0 + other.0,
            self.1 + other.1
        )
    }
}

impl<T> Sub for Vec2<T> where T: Debug + Float {
    type Output = Vec2<T>;
    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::<T>(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> SubAssign for Vec2<T> where T: Debug + Float {
    fn sub_assign(self: &mut Self, other: Vec2<T>) {
        *self = Vec2::<T> (
            self.0 - other.0,
            self.1 - other.1
        )
    }
}

impl<T> Mul<Vec2<T>> for Vec2<T> where T: Debug + Float {
    type Output = T;
    /// Returns the dot-product of the vectors.
    fn mul(self, other: Vec2<T>) -> T {
        self.0 * other.0 + self.1 * other.1
    }
}

impl<T> MulAssign<T> for Vec2<T> where T: Debug + Float {
    /// Mutates the vector by multiplying it with the scalar operand.
    fn mul_assign(&mut self, other: T) {
        *self = Vec2::<T>(self.0 * other, self.1 * other)
    }
}

impl<T> Mul<T> for Vec2<T> where T: Debug + Float {
    type Output = Vec2<T>;
    /// Multiplies the vector with given scalar operand.
    fn mul(self, other: T) -> Vec2<T> {
        Vec2::<T>(self.0 * other, self.1 * other)
    }
}

impl<T> DivAssign<T> for Vec2<T> where T: Debug + Float {
    /// Mutates the vector by dividing it by given scalar.
    fn div_assign(&mut self, other: T) {
        *self = Vec2::<T>(self.0 / other, self.1 / other)
    }
}

impl<T> Div<T> for Vec2<T> where T: Debug + Float {
    type Output = Vec2<T>;
    /// Divides the vector by given scalar operand.
    fn div(self, other: T) -> Vec2<T> {
        Vec2::<T>(self.0 / other, self.1 / other)
    }
}

impl Mul<Vec2<f32>> for f32 {
    type Output = Vec2<f32>;
    fn mul(self, other: Vec2<f32>) -> Vec2<f32> {
        Vec2::<f32>(self * other.0, self * other.1)
    }
}

impl Mul<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;
    fn mul(self, other: Vec2<f64>) -> Vec2<f64> {
        Vec2::<f64>(self * other.0, self * other.1)
    }
}

#[doc(hidden)]
impl AsUniformValue for Vec2<f32> {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec2([ self.0, self.1 ])
    }
}

#[doc(hidden)]
impl AsUniformValue for Vec2<f64> {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::DoubleVec2([ self.0, self.1 ])
    }
}

impl<T> Debug for Vec2<T> where T: Debug + Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({:?}, {:?})", self.0, self.1)
    }
}
