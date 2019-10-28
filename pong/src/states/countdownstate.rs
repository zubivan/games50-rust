use ggez::nalgebra::Point2;

use ggez::Context;

use std::f32;

use std::time::Duration;

use super::gamestate::GameState;
use super::utils;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
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
                Point2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.),
                200.,
            );
        } else {
            utils::draw_text_at_location(
                ctx,
                format!("{}", self.dt.ceil()),
                Point2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.),
                200.,
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
