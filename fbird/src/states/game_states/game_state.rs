use ggez::{graphics, graphics::Image, nalgebra::Point2 as P2, Context};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

use super::utils;

pub struct GameState {
    image: Image,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let image = ggez::graphics::Image::new(ctx, "/bird.png").unwrap();
        GameState { image }
    }
}

impl StateWithTransition for GameState {}

impl Transition for GameState {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        None
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context, _dt: time::Duration) {}
    fn draw(&mut self, ctx: &mut Context) {
        let location = utils::center(
            ctx,
            &self.image,
            P2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.),
        );
        ggez::graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::default().dest(location),
        )
        .unwrap();
    }
}
