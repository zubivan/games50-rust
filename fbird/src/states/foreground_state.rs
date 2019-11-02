use ggez;
use ggez::{graphics::DrawParam, graphics::Image, nalgebra::Point2 as P2, Context};
use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traits::State;

const GROUND_SCROLL_SPEED: f32 = 60.;

pub struct ForegroundState {
    ground_scroll: f32,
    ground: Image,
}

impl ForegroundState {
    pub fn new(ctx: &mut Context) -> Self {
        let ground = ggez::graphics::Image::new(ctx, "/ground.png").unwrap();
        ForegroundState {
            ground_scroll: 0.,
            ground: ground,
        }
    }
}

impl State for ForegroundState {
    fn update(&mut self, _ctx: &mut Context, dt: time::Duration) {
        self.ground_scroll =
            (self.ground_scroll + GROUND_SCROLL_SPEED * dt.as_secs_f32()) % WINDOW_WIDTH;
    }
    fn draw(&mut self, ctx: &mut Context) {
        ggez::graphics::draw(
            ctx,
            &self.ground,
            DrawParam::default().dest(P2::new(
                -self.ground_scroll,
                WINDOW_HEIGHT - self.ground.height() as f32,
            )),
        )
        .unwrap();
    }
}
