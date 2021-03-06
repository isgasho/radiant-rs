use core::{Texture, Renderer, Context, Program, BlendMode, Postprocessor, Color, Point2};

/// A basic postprocessor that applies a Program to the given input once.
///
/// Postprocessors are used with [`Renderer::postprocess()`](../struct.Renderer.html#method.postprocess).
/// The associated type for this postprocessor is `BlendMode` and is expected as second argument
/// to the `postprocess()` method
///
/// # Examples
///
/// ```rust
/// # use radiant_rs::*;
/// # let display = Display::builder().hidden().build().unwrap();
/// # let renderer = Renderer::new(&display).unwrap();
/// # let context = display.context();
/// # let my_layer = Layer::new((1.0, 1.0));
/// # let program_source = "#version 140\nout vec4 f_color;\nvoid main() { f_color = vec4(0.0, 0.0, 0.0, 0.0); }";
/// // Load a shader progam.
/// let my_program = Program::from_string(&context, &program_source).unwrap();
///
/// // Create the postprocessor with the program.
/// let my_postprocessor = postprocessors::Basic::new(&context, my_program, display.dimensions());
///
/// // ... in your renderloop...
/// # display.prepare_frame();
/// renderer.postprocess(&my_postprocessor, &blendmodes::ALPHA, || {
///     renderer.clear(Color::BLACK);
///     renderer.draw_layer(&my_layer, 0);
/// });
/// # display.swap_frame();
/// ```
pub struct Basic {
    source  : Texture,
    program : Program,
}

impl Postprocessor for Basic {
    /// The Basic postprocessor accepts a blendmode as argument to `Renderer::postprocess()`.
    type T = BlendMode;
    fn target(self: &Self) -> &Texture {
        &self.source
    }
    fn draw(self: &Self, renderer: &Renderer, blendmode: &Self::T) {
        renderer.fill().blendmode(*blendmode).program(&self.program).texture(&self.source).draw();
    }
}

impl Basic {
    /// Creates a new instance. The shader can use `sheet*()` to access the input texture.
    pub fn new<T>(context: &Context, program: Program, dimensions: T) -> Self where Point2<u32>: From<T> {

        let (width, height) = Point2::<u32>::from(dimensions);

        let result = Basic {
            source      : Texture::new(&context, width, height),
            program     : program,
        };

        result.source.clear(Color::TRANSPARENT);
        result
    }
}
