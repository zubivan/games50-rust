#[cfg(not(target_arch = "wasm32"))]
extern crate ggez;
#[cfg(target_arch = "wasm32")]
extern crate good_web_game as ggez;

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
