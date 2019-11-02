use ggez;
use ggez::{graphics::DrawParam, graphics::Image, nalgebra::Point2 as P2, Context};
use std::time;

use crate::traits::State;

const BACKGROUND_SCROLL_SPEED: f32 = 30.;

const BACKGROUND_LOOPING_POINT: f32 = 413.;

pub struct BackgroundState {
    background_scroll: f32,
    background: Image,
}

impl BackgroundState {
    pub fn new(ctx: &mut Context) -> Self {
        let background = ggez::graphics::Image::new(ctx, "/background.png").unwrap();
        BackgroundState {
            background_scroll: 0.,
            background: background,
        }
    }
}

impl State for BackgroundState {
    fn update(&mut self, _ctx: &mut Context, dt: time::Duration) {
        self.background_scroll = (self.background_scroll
            + BACKGROUND_SCROLL_SPEED * dt.as_secs_f32())
            % BACKGROUND_LOOPING_POINT;
    }
    fn draw(&mut self, ctx: &mut Context) {
        ggez::graphics::draw(
            ctx,
            &self.background,
            DrawParam::default().dest(P2::new(-self.background_scroll, 0.)),
        )
        .unwrap();
    }
}
