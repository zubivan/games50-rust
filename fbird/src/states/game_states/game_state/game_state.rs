use ggez::{graphics, graphics::Image, nalgebra::Point2 as P2, Context};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

use super::bird::Bird;

use super::super::utils;

pub struct GameState {
    bird: Bird
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        GameState { bird: Bird::new(ctx) }
    }
}

impl StateWithTransition for GameState {}

impl Transition for GameState {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        None
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context, _dt: time::Duration) {
        self.bird.update(ctx, _dt);
    }
    fn draw(&mut self, ctx: &mut Context) {
        self.bird.draw(ctx)
    }
}
