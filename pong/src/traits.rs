use ggez::Context;
use std::time::Duration;

pub trait GameObject {
    fn update(&mut self, ctx: &Context, dt: Duration);
    fn draw(&self, ctx: &mut Context);
}
