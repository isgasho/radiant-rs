use prelude::*;
use core::{self, rendercontext, RenderContext, display, Display, AsUniform, UniformList, Color};
use maths::{Mat4};
use glium;

const SPRITE_INC: &'static str = include_str!("../shader/sprite.inc.fs");
const TEXTURE_INC: &'static str = include_str!("../shader/texture.inc.fs");
const SPRITE_VS: &'static str = include_str!("../shader/sprite.vs");
const TEXTURE_VS: &'static str = include_str!("../shader/texture.vs");

/// A fragment shader program.
pub struct Program {
    uniforms: UniformList,
    sprite_program: glium::Program,
    texture_program: glium::Program,
}

impl Program {
    /// Creates a program from a fragment shader file.
    pub fn from_file(context: &RenderContext, file: &str) -> core::Result<Self> {
        use std::io::Read;
        let mut source = String::new();
        let mut f = File::open(file)?;
        f.read_to_string(&mut source)?;
        Program::from_string(context, &source)
    }
    /// Creates a program from a fragment shader string.
    pub fn from_string(context: &RenderContext, source: &str) -> core::Result<Self> {
        let context = rendercontext::lock(context);
        create(&context.display, source)
    }
    /// Sets a uniform value by name.
    pub fn set_uniform<T>(self: &mut Self, name: &str, value: &T) where T: AsUniform {
        self.uniforms.insert(name, value.as_uniform());
    }
}

/// Creates a new program. Used in rendercontext creation when the full context is not yet available.
pub fn create(display: &Display, source: &str) -> core::Result<Program> {
    let sprite_fs = insert_template(source, SPRITE_INC);
    let texture_fs = insert_template(source, TEXTURE_INC);
    let display_handle = &display::handle(display);
    let dimensions = display.dimensions();
    let mut uniforms = UniformList::new();
    uniforms.insert("view_matrix", Mat4::viewport(dimensions.0 as f32, dimensions.1 as f32).as_uniform());
    uniforms.insert("model_matrix", Mat4::<f32>::identity().as_uniform());
    uniforms.insert("global_color", Color::white().as_uniform());
    Ok(Program {
        uniforms: uniforms,
        sprite_program: create_program(display_handle, SPRITE_VS, &sprite_fs)?,
        texture_program: create_program(display_handle, TEXTURE_VS, &texture_fs)?,
    })
}

/// Private accessor to the sprite fragement shader program.
pub fn sprite(program: &Program) -> &glium::Program {
    &program.sprite_program
}

/// Private accessor to the texture fragement shader program.
pub fn texture(program: &Program) -> &glium::Program {
    &program.texture_program
}

/// Returns immutable uniforms
pub fn uniforms(program: &Program) -> &UniformList {
    &program.uniforms
}

/// Creates a shader program from given vertex- and fragment-shader sources.
fn create_program(display: &glium::Display, vertex_shader: &str, fragment_shader: &str) -> core::Result<glium::Program> {
    use glium::program::ProgramCreationError;
    glium::Program::from_source(display, vertex_shader, fragment_shader, None).map_err(|err| match err {
        ProgramCreationError::CompilationError(message) => { core::Error::ShaderError(format!("Shader compilation failed with: {}", message)) }
        ProgramCreationError::LinkingError(message) => { core::Error::ShaderError(format!("Shader linking failed with: {}", message)) }
        _ => { core::Error::ShaderError("No shader support found".to_string()) }
    })
}

/// Inserts program boilterplate code into the shader source.
fn insert_template(source: &str, template: &str) -> String {
    let mut result = String::new();
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        result.push_str(line);
        result.push_str("\n");
        if line.starts_with("#") {
            result.push_str(template);
        }
    }
    result
}
