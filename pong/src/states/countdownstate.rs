use ggez::nalgebra::Point2;

use ggez::Context;

use std::f32;

use std::time::Duration;

use super::gamestate::GameState;
use super::utils;
use crate::constants::{FONT_SIZE, VIRTUAL_WORLD_HEIGHT, VIRTUAL_WORLD_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

pub struct CountdownState {
    dt: f32,
}

impl CountdownState {
    pub fn new() -> Self {
        CountdownState { dt: 3. }
    }
}

impl StateWithTransition for CountdownState {}

impl State for CountdownState {
    fn update(&mut self, _ctx: &Context, dt: Duration) {
        self.dt -= dt.as_secs_f32();
    }
    fn draw(&self, ctx: &mut Context) {
        if self.dt <= 0. {
            utils::draw_text_at_location(
                ctx,
                String::from("GO!"),
                Point2::new(VIRTUAL_WORLD_WIDTH / 2., VIRTUAL_WORLD_HEIGHT / 2.),
                FONT_SIZE,
            );
        } else {
            utils::draw_text_at_location(
                ctx,
                format!("{}", self.dt.ceil()),
                Point2::new(VIRTUAL_WORLD_WIDTH / 2., VIRTUAL_WORLD_HEIGHT / 2.),
                FONT_SIZE,
            );
        }
    }
}

impl Transition for CountdownState {
    fn transition(&self) -> Option<Box<dyn StateWithTransition>> {
        if self.dt <= -0.5 {
            Some(Box::new(GameState::new()))
        } else {
            None
        }
    }
}
