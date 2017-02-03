use prelude::*;
use glium;
use core::{Display, rendercontext, RenderContext, RenderContextData, texture, Texture, layer, Layer, blendmode, BlendMode, scene, Color, RenderTarget};
use maths::{Vec2, Point2, Mat4};

/// A renderer is used to render [`Layer`](struct.Layer.html)s or [`Scene`](struct.Scene.html)s to the
/// [`Display`](struct.Display.html).
///
/// The renderer itself is not thread-safe. Instead, draw or write onto layers (from any one or
/// more threads)  and present those layers using the renderer once your threads have concluded
/// drawing.
///
/// Alternatively to directly drawing on layers, [`Scene`](struct.Scene.html) provides a higher
/// level abstraction.
#[derive(Clone)]
pub struct Renderer {
    context: RenderContext,
}

impl<'a> Renderer {

    /// Returns a new renderer instance.
    pub fn new(display: &Display) -> Self {

        let context_data = RenderContextData::new(display, rendercontext::INITIAL_CAPACITY);

        Renderer {
            context: rendercontext::new(context_data),
        }
    }

    /// Returns a reference to the renderers' context. The [`RenderContext`](struct.RenderContext)
    /// is thread-safe and required by [`Font`](struct.Font) and [`Sprite`](struct.Sprite) to
    /// create new instances.
    pub fn context(self: &Self) -> RenderContext {
        self.context.clone()
    }

    /// Sets a new rendering target. Valid targets are the display and textures.
    pub fn set_target<T>(self: &Self, target: &T) where T: RenderTarget {
        let mut context = rendercontext::lock(&self.context);
        context.render_target = target.get_target().clone();
    }

    /// Clears the current target.
    pub fn clear(self: &Self, color: &Color) {
        let context = rendercontext::lock(&self.context);
        context.render_target.clear(color);
    }

    /// Draws given scene to the current target..
    pub fn draw_scene(self: &Self, scene: &scene::Scene, per_frame_multiplier: f32) -> &Self {
        scene::draw(scene, self, per_frame_multiplier);
        self
    }

    /// Draws given layer to the current target..
    pub fn draw_layer(self: &Self, layer: &Layer) -> &Self {

        // open context

        let mut context = rendercontext::lock(&self.context);
        let mut context = context.deref_mut();

        // update sprite texture arrays, font texture and vertex buffer as required

        context.update_tex_array();
        context.update_font_cache();
        let (vertex_buffer, num_vertices) = layer::upload(&layer, context);
        context.update_index_buffer(num_vertices / 4);

        // draw the layer, unless it is empty

        if num_vertices > 0 {

            // set up uniforms

            let uniforms = uniform! {
                view_matrix     : *layer.view_matrix().deref_mut(),
                model_matrix    : *layer.model_matrix().deref_mut(),
                global_color    : *layer.color().deref_mut(),
                font_cache      : context.font_texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                tex1            : &context.tex_array[1].data,   // 32
                tex2            : &context.tex_array[2].data,   // 64
                tex3            : &context.tex_array[3].data,   // 128
                tex4            : &context.tex_array[4].data,   // 256
                tex5            : &context.tex_array[5].data,   // 512
            };

            // set up draw parameters for given blend options

            let draw_parameters = glium::draw_parameters::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
                blend           : blendmode::access_blendmode(layer.blendmode().deref_mut()),
                .. Default::default()
            };

            // draw up to container.size

            let ib_slice = context.index_buffer.slice(0..num_vertices as usize / 4 * 6).unwrap();
            context.render_target.draw(vertex_buffer.as_ref().unwrap(), &ib_slice, &context.program, &uniforms, &draw_parameters).unwrap();
        }

        self
    }

    /* /// Draws given texture to the current target.
    pub fn draw_texture(self: &Self, texture: &Texture, position: Point2, size: Vec2, blendmode: BlendMode, time: f32, horizontal: bool) -> &Self {

        // open context

        let mut context = rendercontext::lock(&self.context);
        let mut context = context.deref_mut();

        context.update_index_buffer(1);

        let texture = texture::handle(texture);

        // set up uniforms

        let uniforms = uniform! {
            view_matrix     : Mat4::<f32>::viewport(640.0, 480.0),
            global_color    : Color::white(),
            offset          : position,
            size            : size,
            tex             : texture,
            time            : time,
            horizontal      : horizontal,
        };

        // set up draw parameters for given blend options

        let draw_parameters = glium::draw_parameters::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            blend           : blendmode::access_blendmode(&blendmode),
            .. Default::default()
        };

        // draw up to container.size

        let ib_slice = context.index_buffer.slice(0..6).unwrap();
        context.render_target.draw(&context.vertex_buffer_single, &ib_slice, &context.program_single, &uniforms, &draw_parameters).unwrap();

        self
    }*/
}
/// returns the appropriate bucket_id and padded texture size for the given texture size
pub fn bucket_info(width: u32, height: u32) -> (u32, u32) {
    let ln2 = (cmp::max(width, height) as f32).log2().ceil() as u32;
    let size = 2u32.pow(ln2);
    // skip first five sizes 1x1 to 16x16, use id 0 for font-cache
    let bucket_id = cmp::max(0, ln2 - 4 + 1);
    assert!(bucket_id < rendercontext::NUM_BUCKETS as u32, "texture size exceeded configured maximum");
    (bucket_id, size)
}
