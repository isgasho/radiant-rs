mod blendmode;
mod display;
mod input;
mod layer;
mod renderer;
mod sprite;
mod font;
mod rendercontext;
mod scene;
mod color;
mod monitor;
mod texture;
mod program;
mod uniform;

pub use self::blendmode::{blendmodes, BlendMode};
pub use self::input::{Input, InputId, InputState, InputIterator, InputUpIterator, InputDownIterator};
pub use self::display::{Display, DisplayInfo};
pub use self::sprite::Sprite;
pub use self::renderer::Renderer;
pub use self::font::{Font, FontInfo, FontCache};
pub use self::layer::Layer;
pub use self::rendercontext::{RenderContext, RenderContextData, RenderContextTexture, RenderContextTextureArray};
pub use self::color::Color;
pub use self::scene::*;
pub use self::monitor::Monitor;
pub use self::texture::{Texture, TextureFilter, TextureWrap};
pub use self::program::Program;
pub use self::uniform::{Uniform, AsUniform, UniformList, GliumUniform};

use glium::index::IndicesSource;
use glium::uniforms::Uniforms;
use glium::vertex::MultiVerticesSource;
use glium::{self, Surface, DrawParameters};
use image;
use prelude::*;

/// An enum of render target type instances.
#[derive(Clone)]
pub enum RenderTarget {
    None,
    Display(Display),
    Texture(Texture),
}

impl RenderTarget {
    /// Draws to the target.
    fn draw<'b, 'v, V, I, U>(self: &Self, vb: V, ib: I, program: &glium::Program, uniforms: &U, draw_parameters: &DrawParameters)
        where I: Into<IndicesSource<'b>>, U: Uniforms, V: MultiVerticesSource<'v> {

        match *self {
            RenderTarget::Display(ref display) => {
                display::draw(display, vb, ib, program, uniforms, draw_parameters).unwrap()
            }
            RenderTarget::Texture(ref texture) => {
                texture::handle(texture).as_surface().draw(vb, ib, program, uniforms, draw_parameters).unwrap()
            }
            RenderTarget::None => { }
        }
    }
    /// Clears the target.
    fn clear(self: &Self, color: Color) {
        match *self {
            RenderTarget::Display(ref display) => {
                display::clear(display, color);
            },
            RenderTarget::Texture(ref texture) => {
                let Color(r, g, b, a) = color;
                texture::handle(texture).as_surface().clear_color(r, g, b, a);
            }
            RenderTarget::None => { }
        }
    }
}

/// A target for rendering.
pub trait AsRenderTarget {
    /// Returns RenderTarget enum containing a texture or a frame.
    fn as_render_target(self: &Self) -> RenderTarget;
}

/// A custom postprocessor. Note: API is likely to change!
///
/// Postprocessing happens in three steps:
///
/// - first, `target()` is invoked and expected to return the input texture target (from
/// where the postprocessor intends to read input data).
/// - `process()` is invoked and expected to perform the neccessary processing
/// **excluding** the final draw operation.
/// - `draw()` is invoked. At this point the renderer has already restored the drawing
/// target so that this method is only required to draw the postprocessing result
/// to the current target.
pub trait Postprocessor {
    /// Returns a texture and blendmode. The renderer will draw to this texture
    /// using the returned blendmode.
    fn target(self: &mut Self) -> (&Texture, BlendMode);
    /// Processes input data. Simple postprocessors may not need to implement this.
    #[allow(unused_variables)]
    fn process(self: &mut Self, renderer: &Renderer) { }
    /// Draws final result to current target. Expected to use given blendmode.
    fn draw(self: &mut Self, renderer: &Renderer, blendmode: BlendMode);
}

/// Radiant errors.
#[derive(Debug)]
pub enum Error {
    ImageError(String),
    ShaderError(String),
    IoError(io::Error),
    Failed,
}

impl From<io::Error> for Error {
    /// Converts io error to radiant error
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<image::ImageError> for Error {
    /// Converts image error to radiant error
    fn from(error: image::ImageError) -> Error {
        use image::ImageError;
        match error {
            ImageError::IoError(error)          => { Error::IoError(error) }
            ImageError::FormatError(error)      => { Error::ImageError(format!("Image format error: {}", error)) }
            ImageError::UnsupportedError(error) => { Error::ImageError(format!("Image unsupported: {}", error)) }
            ImageError::UnsupportedColor(_)     => { Error::ImageError("Unsupported colorformat".to_string()) }
            _                                   => { Error::ImageError("Unknown image error".to_string()) }
        }
    }
}

/// Radiant result.
pub type Result<T> = result::Result<T, Error>;
