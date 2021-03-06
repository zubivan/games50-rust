use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;
use ggez::Context;

use std::time::Duration;

use super::countdownstate::CountdownState;
use super::utils;

use crate::constants::{FONT_SIZE, VIRTUAL_WORLD_HEIGHT, VIRTUAL_WORLD_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

pub struct GameOverState {
    winner: Winner,
    ready: bool,
}

pub enum Winner {
    PlayerOne,
    PlayerTwo,
}

impl GameOverState {
    pub fn new(winner: Winner) -> Self {
        GameOverState {
            winner: winner,
            ready: false,
        }
    }
}

impl StateWithTransition for GameOverState {}

impl State for GameOverState {
    fn update(&mut self, ctx: &Context, _dt: Duration) {
        self.ready = keyboard::is_key_pressed(ctx, KeyCode::Space);
    }
    fn draw(&self, ctx: &mut Context) {
        utils::draw_text_at_location(
            ctx,
            format!(
                "Player {} won!",
                match self.winner {
                    Winner::PlayerOne => 1,
                    Winner::PlayerTwo => 2,
                }
            ),
            Point2::new(VIRTUAL_WORLD_WIDTH / 2., VIRTUAL_WORLD_HEIGHT / 2.),
            FONT_SIZE,
        );
    }
}

impl Transition for GameOverState {
    fn transition(&self) -> Option<Box<dyn StateWithTransition>> {
        if self.ready {
            Some(Box::new(CountdownState::new()))
        } else {
            None
        }
    }
}
