mod mat4;
mod vec2;
mod vec3;
mod angle;
mod rect;
use prelude::*;

pub use self::mat4::Mat4;
pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::angle::Angle;
pub use self::rect::{Point2, Rect};

/// A 3-dimensional point.
pub type Point3<T = f32> = Vec3<T>;

// required due to #26953
#[allow(non_snake_case)]
pub fn Point3<T: Debug + Float>(x: T, y: T, z: T) -> Point3<T> {
    Vec3(x, y, z)
}

/// Trait for values that can be converted to a vector.
pub trait VecType<T: Copy + fmt::Debug + Float> {
    /// Returns the given value as a Vec3
    fn as_vec3(self: &Self, neutral: T) -> Vec3<T>;
}

impl<T: Copy + fmt::Debug + Float> VecType<T> for (T, T, T) {
    fn as_vec3(self: &Self, _: T) -> Vec3<T> {
        Vec3::<T>(self.0, self.1, self.2)
    }
}

impl<T: Copy + fmt::Debug + Float> VecType<T> for (T, T) {
    fn as_vec3(self: &Self, neutral: T) -> Vec3<T> {
        Vec3::<T>(self.0, self.1, neutral)
    }
}

impl<T: Copy + fmt::Debug + Float> VecType<T> for T {
    fn as_vec3(self: &Self, _: T) -> Vec3<T> {
        Vec3::<T>(*self, *self, *self)
    }
}
