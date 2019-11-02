use ggez;
use ggez::{Context, graphics::Image, graphics::DrawParam, nalgebra::Point2 as P2};
use std::time;

use crate::traits::State;

const BACKGROUND_SCROLL_SPEED: f32 = 30.;
const GROUND_SCROLL_SPEED: f32 = 60.;


const BACKGROUND_LOOPING_POINT: f32 = 413.;

pub struct BackgroundState {
    background_scroll: f32,
    ground_scroll: f32,
    background: Image,
    ground: Image
}

impl BackgroundState {
    pub fn new(ctx: &mut Context) -> Self {
        let background = ggez::graphics::Image::new(ctx, "/background.png").unwrap();
        let ground = ggez::graphics::Image::new(ctx, "/ground.png").unwrap();
        BackgroundState {
            background_scroll: 0.,
            ground_scroll: 0.,
            background: background,
            ground: ground
        }
    }
}

impl State for BackgroundState {    
    fn update(&mut self, ctx: &mut Context, dt: time::Duration) {
        self.background_scroll = (self.background_scroll + BACKGROUND_SCROLL_SPEED * dt.as_secs_f32()) % BACKGROUND_LOOPING_POINT;
        //self.ground_scroll = (self.ground_scroll + GROUND_SCROLL_SPEED * .as_secs_f32()) % VIRTUAL_WIDTH;
    }
    fn draw(&mut self, ctx: &mut Context) {
        ggez::graphics::draw(ctx, 
        &self.background, 
        DrawParam::default().dest(P2::new(-self.background_scroll, 0.)));
    }
}