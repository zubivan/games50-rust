use ggez::Context;
use std::time::Duration;

pub trait StateWithTransition: State + Transition {}

pub trait State {
    fn update(&mut self, ctx: &Context, dt: Duration);
    fn draw(&self, ctx: &mut Context);
}

pub trait Transition {
    fn transition(&self) -> Option<Box<dyn StateWithTransition>>;
}
