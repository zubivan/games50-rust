#[cfg(not(target_arch = "wasm32"))]
extern crate ggez;
#[cfg(target_arch = "wasm32")]
extern crate good_web_game as ggez;

use ggez::graphics;
use ggez::graphics::{DrawParam, Font, Text};
use ggez::nalgebra::Point2;
use ggez::Context;

pub fn draw_text_at_location(
    ctx: &mut Context,
    text: String,
    location: Point2<f32>,
    font_size: f32,
) {
    let font = Font::new(ctx, "/fonts/font.ttf").unwrap();
    let text = Text::new((text, font, font_size));

    let (text_width, text_height) = measure_text(ctx, &text);
    let location = Point2::new(
        location.x - (text_width / 2.),
        location.y - (text_height / 2.),
    );
    graphics::draw(ctx, &text, DrawParam::default().dest(location)).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn measure_text(ctx: &mut Context, text: &Text) -> (f32, f32) {
    (text.width(ctx) as f32, text.height(ctx) as f32)
}

#[cfg(target_arch = "wasm32")]
fn measure_text(ctx: &mut Context, text: &Text) -> (f32, f32) {
    use ggez::graphics::Drawable;
    
    let dimensions: Option<graphics::Rect> = text.dimensions(ctx);
    let dimensions = dimensions.unwrap();
    (dimensions.w, dimensions.h)
}
