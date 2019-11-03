use ggez::{Context, graphics::Text, graphics::Image, nalgebra::Point2 as P2};

pub fn center(ctx: &mut Context, text: &dyn Measurable, centrum_location: P2<f32>) -> P2<f32> {
    let x = centrum_location.x - text.width(ctx) as f32/ 2.;
    let y = centrum_location.y - text.height(ctx)  as f32/ 2.;
    P2::new(x, y)
}

pub trait Measurable {
    fn height(&self, ctx: &mut Context) -> f32;
    fn width(&self, ctx: &mut Context) -> f32;
}

impl Measurable for Text {
    fn height(&self, ctx: &mut Context) -> f32 {
        self.height(ctx) as f32
    }
    fn width(&self, ctx: &mut Context) -> f32 {
        self.width(ctx) as f32
    }
}

impl Measurable for Image {
    fn height(&self, _ctx: &mut Context) -> f32 {
        self.height() as f32
    }
    fn width(&self, _ctx: &mut Context) -> f32 {
        self.width() as f32
    }
}