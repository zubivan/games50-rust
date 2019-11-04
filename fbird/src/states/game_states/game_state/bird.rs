use ggez::Context;
use ggez::event::{KeyCode};
use ggez::{graphics, graphics::{Image}};
use ggez::nalgebra::Point2 as P2;
use ggez::input::{keyboard};

use std::time;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

const GRAVITY: f32 = 20.;

pub struct Bird {
    x: f32,
    y: f32,
    d_y: f32,
    image: Image,
}

impl Bird {
    pub fn new(ctx: &mut Context) -> Self {
        let image = ggez::graphics::Image::new(ctx, "/bird.png").unwrap();
        Bird {
            x: WINDOW_WIDTH / 2.,
            y: WINDOW_HEIGHT / 2.,
            d_y: 0.,
            image: image
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: time::Duration) {
        self.d_y += GRAVITY * dt.as_secs_f32();
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.d_y = -5.;
        }

        self.y += self.d_y;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let location = super::super::utils::center(
            ctx,
            &self.image,
            P2::new(self.x, self.y),
        );
        ggez::graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::default().dest(location),
        )
        .unwrap();
    }
}
