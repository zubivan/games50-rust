use ggez::{Context, graphics::Text, nalgebra::Point2 as P2};

pub fn center(ctx: &mut Context, text: &Text, centrum_location: P2<f32>) -> P2<f32> {
    let x = centrum_location.x - text.width(ctx) as f32/ 2.;
    let y = centrum_location.y - text.height(ctx)  as f32/ 2.;
    P2::new(x, y)
}
