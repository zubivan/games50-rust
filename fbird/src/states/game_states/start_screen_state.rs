use ggez::{graphics, Context};
use ggez::{
    input::keyboard::{is_key_pressed, KeyCode},
    nalgebra::Point2 as P2,
};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

use super::utils;

pub struct StartScreenState {
    ready: bool,
}

impl StartScreenState {
    pub fn new(_ctx: &mut Context) -> Self {
        StartScreenState { ready: false }
    }
}

impl StateWithTransition for StartScreenState {}

impl Transition for StartScreenState {
    fn transition(&mut self, _ctx: &mut Context) -> Option<Box<dyn StateWithTransition>> {
        if self.ready {
            Some(Box::new(
                super::countdown_screen_state::StartScreenState::new(),
            ))
        } else {
            None
        }
    }
}

impl State for StartScreenState {
    fn update(&mut self, ctx: &mut Context, _dt: time::Duration) {
        self.ready = is_key_pressed(ctx, KeyCode::Space);
    }
    fn draw(&mut self, ctx: &mut Context) {
        let text = ggez::graphics::Text::new("Press space to start");
        let location = utils::center(ctx, &text, P2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.));
        ggez::graphics::draw(ctx, &text, graphics::DrawParam::default().dest(location)).unwrap();
    }
}
