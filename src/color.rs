use prelude::*;
use glium::uniforms::{AsUniformValue, UniformValue};
use glium::vertex::{Attribute, AttributeType};

/// A color value consisting of four floating point values for the color channels red, green, blue
/// and alpha.
///
/// Various drawing methods accept color instances to be used as multiplicators in the drawing
/// process, i.e. [`Sprite::draw()`](struct.Sprite.html#method.draw) allows multiplying the sprite-
/// texture's color channels by given color.
#[derive(Copy, Clone, Default)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {

    //const RED: Color = Color(1.0, 0, 0, 1.0);

    /// Creates a new instance with a channels set to zero.
    pub fn transparent() -> Color {
        Color(0.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new instance with color channels set to one and the alpha channel set to given value.
    pub fn alpha(alpha: f32) -> Color {
        Color(1.0, 1.0, 1.0, alpha)
    }

    /// Creates a new instance with color channels set to zero and the alpha channel set to given value.
    pub fn alpha_mask(alpha: f32) -> Color {
        Color(0.0, 0.0, 0.0, alpha)
    }

    /// Creates a new instance with all channels set to given value.
    pub fn alpha_pm(alpha: f32) -> Color {
        Color(alpha, alpha, alpha, alpha)
    }

    /// Creates a new instance with color channels set to given value and the alpha channel set to one.
    pub fn lightness(value: f32) -> Color {
        Color(value, value, value, 1.0)
    }

    /// Creates a new instance from given color-temperature (~1000 to ~40000K).
    ///
    /// Based on http://www.tannerhelland.com/4435/convert-temperature-rgb-algorithm-code/
    pub fn temperature(temperature: f32, alpha: f32) -> Color {

        let value = (temperature / 100.0).floor();
        let red;
        let green;
        let blue;

        if value <= 66.0 {
            red = 255;
            green = (99.4708025861 * value.ln() - 161.1195681661) as i32;
        } else {
            red = (329.698727466 * (value - 60.0).powf(-0.1332047592)) as i32;
            green = (288.1221695283 * (value - 60.0).powf(-0.0755148492)) as i32;
        }

        if value >= 66.0 {
            blue = 255;
        } else if value <= 19.0 {
            blue = 0;
        } else {
            blue = (138.5177312231 * (value - 10.0).ln() - 305.0447927307) as i32;
        }

        Color(
            cmp::max(0, cmp::min(255, red)) as f32 / 255.0,
            cmp::max(0, cmp::min(255, green)) as f32 / 255.0,
            cmp::max(0, cmp::min(255, blue)) as f32 / 255.0,
            alpha
        )
    }

    /// Returns value of the instance's red channel.
    pub fn r(&self) -> f32 {
        self.0
    }

    /// Returns value of the instance's green channel.
    pub fn g(&self) -> f32 {
        self.1
    }

    /// Returns value of the instance's blue channel.
    pub fn b(&self) -> f32 {
        self.2
    }

    /// Returns value of the instance's alpha channel.
    pub fn a(&self) -> f32 {
        self.3
    }

    /// Sets the instance's channels from another color object.
    pub fn set(&mut self, value: Color) {
        self.0 = value.0;
        self.1 = value.1;
        self.2 = value.2;
        self.3 = value.3;
    }

    /// Sets a value for the instance's red channel
    pub fn set_r(&mut self, value: f32) -> Color {
        self.0 = value;
        *self
    }

    /// Sets a value for the instance's green channel.
    pub fn set_g(&mut self, value: f32) -> Color {
        self.1 = value;
        *self
    }

    /// Sets a value for the instance's blue channel.
    pub fn set_b(&mut self, value: f32) -> Color {
        self.2 = value;
        *self
    }

    /// Sets a value for the instance's alpha channel.
    pub fn set_a(&mut self, value: f32) -> Color {
        self.3 = value;
        *self
    }

    /// Multiplies the instance's color channels by given scaling factor. Does not modify alpha.
    pub fn scale(&mut self, scaling: f32) -> Color {
        self.0 *= scaling;
        self.1 *= scaling;
        self.2 *= scaling;
        *self
    }

    /// Returns the instance's channels as a tuple.
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0, self.1, self.2, self.3)
    }

    pub fn white() -> Color {
        Color(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color(0.0, 0.0, 0.0, 1.0)
    }

    pub fn red() -> Color {
        Color(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Color {
        Color(0.0, 0.0, 1.0, 1.0)
    }

    pub fn yellow() -> Color {
        Color(1.0, 1.0, 0.0, 1.0)
    }

    pub fn cyan() -> Color {
        Color(0.0, 1.0, 1.0, 1.0)
    }

    pub fn purple() -> Color {
        Color(1.0, 0.0, 1.0, 1.0)
    }
}

#[doc(hidden)]
unsafe impl Attribute for Color {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

#[doc(hidden)]
impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4([ self.0, self.1, self.2, self.3 ])
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}
