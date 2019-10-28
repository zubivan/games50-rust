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
    let location = Point2::new(
        location.x - (text.width(ctx) as f32 / 2.),
        location.y - (text.height(ctx) as f32 / 2.),
    );
    graphics::draw(ctx, &text, DrawParam::default().dest(location)).unwrap();
}
