use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;

use ggez::Context;

use super::countdownstate::CountdownState;
use super::utils;

use std::time::Duration;

use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

pub struct StartScreenState {
    pub ready: bool,
}

impl StateWithTransition for StartScreenState {}

impl State for StartScreenState {
    fn update(&mut self, ctx: &Context, _dt: Duration) {
        self.ready = keyboard::is_key_pressed(ctx, KeyCode::Space);
    }
    fn draw(&self, ctx: &mut Context) {
        utils::draw_text_at_location(
            ctx,
            String::from("PRESS SPACE TO START"),
            Point2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.),
            80.,
        );
    }
}

impl Transition for StartScreenState {
    fn transition(&self) -> Option<Box<dyn StateWithTransition>> {
        if self.ready {
            Some(Box::new(CountdownState::new()))
        } else {
            None
        }
    }
}
