use ggez::nalgebra::Point2 as P2;
use ggez::{graphics, Context};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

use super::utils;

pub struct StartScreenState {
    current_count: f32,
}

impl StartScreenState {
    pub fn new() -> Self {
        StartScreenState { current_count: 3. }
    }
}

impl StateWithTransition for StartScreenState {}

impl Transition for StartScreenState {
    fn transition(&mut self) -> Option<Box<dyn StateWithTransition>> {
        if self.current_count <= -0.5 {
            Some(Box::from(super::game_state::GameState::new()))
        } else {
            None
        }
    }
}

impl State for StartScreenState {
    fn update(&mut self, _ctx: &mut Context, dt: time::Duration) {
        self.current_count -= dt.as_secs_f32();
    }
    fn draw(&mut self, ctx: &mut Context) {
        let text = if self.current_count <= 0. {
            ggez::graphics::Text::new("GO!")
        } else {
            ggez::graphics::Text::new(format!("{}", self.current_count.ceil()))
        };

        let location = utils::center(ctx, &text, P2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.));
        ggez::graphics::draw(ctx, &text, graphics::DrawParam::default().dest(location)).unwrap();
    }
}
