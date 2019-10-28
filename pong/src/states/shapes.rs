use ggez::graphics;
use ggez::graphics::Rect;
use ggez::graphics::{DrawParam, WHITE};
use ggez::nalgebra as na;

use ggez::Context;

use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use std::time::Duration;

pub trait GameObject {
    fn update(&mut self, ctx: &Context, dt: Duration);
    fn draw(&self, ctx: &mut Context);
}

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub d_x: f32,
    pub d_y: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, d_x: f32, d_y: f32, radius: f32) -> Self {
        Ball {
            x,
            y,
            d_x,
            d_y,
            radius,
        }
    }
}

impl GameObject for Ball {
    fn update(&mut self, _ctx: &Context, dt: Duration) {
        self.x += self.d_x * dt.as_secs_f32();
        self.y += self.d_y * dt.as_secs_f32();
    }
    fn draw(&self, ctx: &mut Context) {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.x, self.y),
            self.radius,
            2.0,
            WHITE,
        )
        .unwrap();

        graphics::draw(ctx, &circle, DrawParam::default()).unwrap();
    }
}

const PADDLE_SPEED: i32 = 300;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub heigh: f32,
    up_key: KeyCode,
    down_key: KeyCode,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, heigh: f32, up_key: KeyCode, down_key: KeyCode) -> Self {
        Paddle {
            x,
            y,
            width,
            heigh,
            up_key,
            down_key,
        }
    }
}

impl GameObject for Paddle {
    fn update(&mut self, ctx: &Context, dt: Duration) {
        let d_y = if keyboard::is_key_pressed(ctx, self.up_key) {
            -PADDLE_SPEED as f32
        } else if keyboard::is_key_pressed(ctx, self.down_key) {
            PADDLE_SPEED as f32
        } else {
            0.
        };

        self.y += d_y * dt.as_secs_f32();
    }

    fn draw(&self, ctx: &mut Context) {
        let rect = Rect::new(self.x, self.y, self.width, self.heigh);
        let rect =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, WHITE).unwrap();
        graphics::draw(ctx, &rect, DrawParam::default()).unwrap();
    }
}
