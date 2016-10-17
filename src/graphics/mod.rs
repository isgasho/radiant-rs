mod blendmode;
mod display;
mod input;
mod layer;
mod renderer;
mod sprite;
mod font;

pub use self::blendmode::{blendmodes, BlendMode};
pub use self::input::Input;
pub use self::display::{DisplayInfo, Monitor};
pub use self::sprite::Sprite;
pub use self::renderer::Renderer;
pub use self::font::{Font, FontInfo, FontCache};

use prelude::*;
use glium;
use color::Color;
use maths::Mat4;
use avec::AVec;

#[derive(Clone)]
pub struct Display {
    handle: glium::Display,
}

pub struct LayerBufferContainer {
    lid     : usize,
    size    : usize,
    vb      : glium::VertexBuffer<Vertex>,
}

pub struct RenderContextTextureArray<'a> {
    dirty   : bool,
    data    : glium::texture::Texture2dArray,
    raw     : Vec<glium::texture::RawImage2d<'a, u8>>,
}

impl<'a> RenderContextTextureArray<'a> {
    pub fn new(display: &Display) -> Self {
        RenderContextTextureArray {
            dirty   : false,
            data    : glium::texture::Texture2dArray::empty(&display.handle, 2, 2, 1).unwrap(),
            raw     : Vec::new(),
        }
    }
}

pub struct RenderContextData<'a> {
    index_buffer    : glium::IndexBuffer<u32>,
    program         : glium::Program,
    tex_array       : Vec<RenderContextTextureArray<'a>>,
    target          : Option<glium::Frame>,
    display         : Display,
    layer_buffers   : HashMap<usize, LayerBufferContainer>,
    font_cache      : font::FontCache,
    font_texture    : glium::texture::Texture2d,
}

pub struct RenderContext<'a> (Mutex<RenderContextData<'a>>);
unsafe impl<'a> Send for RenderContext<'a> { }
unsafe impl<'a> Sync for RenderContext<'a> { }

impl<'a> RenderContext<'a> {
    fn new(data: RenderContextData) -> RenderContext {
        RenderContext (Mutex::new(data))
    }
    fn lock(self: &Self) -> MutexGuard<RenderContextData<'a>> {
        self.0.lock().unwrap()
    }
}


pub struct Layer {
    view_matrix : Mutex<Mat4<f32>>,
    model_matrix: Mutex<Mat4<f32>>,
    blend       : Mutex<BlendMode>,
    color       : Mutex<Color>,
    gid         : usize,
    lid         : AtomicUsize,
	vertex_data : AVec<Vertex>,
}

#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Copy, Clone)]
pub struct Rect (Point, Point);
impl Rect {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
        Rect(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
    }
}

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    position    : [f32; 2],
    offset      : [f32; 2],
    rotation    : f32,
    color       : Color,
    bucket_id   : u32,
    texture_id  : u32,
    texture_uv  : [f32; 2],
}
implement_vertex!(Vertex, position, offset, rotation, color, bucket_id, texture_id, texture_uv);
