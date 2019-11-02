use ggez::Context;
use std::time;

pub trait State {
    fn update(&mut self, ctx: &mut Context, dt: time::Duration);
    fn draw(&mut self, ctx: &mut Context);
}