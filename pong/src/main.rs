use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::graphics::{DrawParam, BLACK, WHITE};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

use std::time;

mod traits;
use traits::GameObject;

mod shapes;
use shapes::{Ball, Paddle};

struct MainState {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            ball: Ball::new(200., 200., 10., 10., 10.),
            player1: Paddle::new(10., 20., 20., 100., KeyCode::W, KeyCode::S),
            player2: Paddle::new(770., 20., 20., 100., KeyCode::Up, KeyCode::Down),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::delta(ctx);
        self.ball.update(ctx, dt);
        self.player1.update(ctx, dt);
        self.player2.update(ctx, dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);
        self.ball.draw(ctx);
        self.player1.draw(ctx);
        self.player2.draw(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("Pong", "Ivan Zub (zub.ivan@gmail.com)");
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, events_loop, state)
}
