use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;

use ggez::Context;

use super::countdownstate::CountdownState;
use super::utils;

use std::time::Duration;

use crate::constants::{SMALL_FONT_SIZE, VIRTUAL_WORLD_HEIGHT, VIRTUAL_WORLD_WIDTH};
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
            Point2::new(VIRTUAL_WORLD_WIDTH / 2., VIRTUAL_WORLD_HEIGHT / 2.),
            SMALL_FONT_SIZE,
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
