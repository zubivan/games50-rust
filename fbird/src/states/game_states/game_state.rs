use ggez::nalgebra::Point2 as P2;
use ggez::{graphics, Context};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

use super::utils;

pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }
}

impl StateWithTransition for GameState {}

impl Transition for GameState {
    fn transition(&mut self) -> Option<Box<dyn StateWithTransition>> {
        None
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context, dt: time::Duration) {}
    fn draw(&mut self, ctx: &mut Context) {
        let text = ggez::graphics::Text::new("PLAY!");
        let location = utils::center(ctx, &text, P2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.));
        ggez::graphics::draw(ctx, &text, graphics::DrawParam::default().dest(location)).unwrap();
    }
}
