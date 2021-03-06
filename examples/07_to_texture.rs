extern crate radiant_rs;
extern crate radiant_utils as ru;
use radiant_rs::{Display, Renderer, Layer, Sprite, Color, Texture, TextureFilter, blendmodes};
use ru::Matrix;

pub fn main() {
    let display = Display::builder().dimensions((640, 480)).vsync().title("Drawing to textures example").build().unwrap();
    let renderer = Renderer::new(&display).unwrap();
    let sprite = Sprite::from_file(display.context(), r"examples/res/sprites/sparkles_64x64x1.png").unwrap();
    let layer = Layer::new((320., 240.));
    layer.set_blendmode(blendmodes::LIGHTEN);

    sprite.draw(&layer, 0, (160., 120.), Color::WHITE);
    sprite.draw(&layer, 0, (130., 100.), Color::RED);
    sprite.draw(&layer, 0, (190., 100.), Color::GREEN);
    sprite.draw(&layer, 0, (160., 155.), Color::BLUE);

    // A texture. Each frame we'll draw the sprites to "surface", then blended with
    // a low opacity black to make old contents slowly disappear.
    let surface = Texture::new(display.context(), 640, 480);

    ru::renderloop(|frame| {
        display.clear_frame(Color::BLACK);

        // Rotate the sprite matrices (this uses the fairyjar::Matrix trait)
        layer.view_matrix().rotate_at((160., 120.), frame.delta_f32);
        layer.model_matrix().rotate(frame.delta_f32 * 1.1);

        // Drawing within Renderer::render_to() redirects the output to the given rendertarget.
        // First we draw the sprites, then we blend the low opacity black on top (to fade previously drawn contents)
        renderer.render_to(&surface, || {
            renderer.draw_layer(&layer, 0);
            renderer.fill().blendmode(blendmodes::ALPHA).color(Color(0., 0., 0., 0.04)).draw();
        });

        if (frame.elapsed_f32 / 1.5) as u32 % 2 == 0 {
            // Copies surface to the display.
            renderer.copy_from(&surface, TextureFilter::Linear);
        } else {
            // Draw the sprites to display.
            renderer.draw_layer(&layer, 0);
        }

        // Draw a small thumbnail of surface
        renderer.copy_rect_from(&surface, ((0, 0), (640, 480)), ((512, 384), (128, 96)), TextureFilter::Linear);

        display.swap_frame();
        !display.poll_events().was_closed()
    });
}
