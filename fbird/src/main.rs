use ggez;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use std::path;

mod constants;
mod traits;

mod states;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

struct MainState {
    // state: Box<dyn traits::State>,
    background_state: Box<dyn traits::State>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        MainState {
            background_state: Box::from(states::background_state::BackgroundState::new(ctx))
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::delta(ctx);
        // self.state.update(ctx, dt);
        self.background_state.update(ctx, dt);
        // if let Some(new_state) = self.state.transition() {
        //     self.state = new_state
        // };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.background_state.draw(ctx);
        //self.state.draw(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let window_mode =
        WindowMode::default()
        .resizable(true)
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    let cb = ggez::ContextBuilder::new("FBird", "Ivan Zub (zub.ivan@gmail.com)")
        .window_mode(window_mode)
        .add_resource_path(path::PathBuf::from("./assets"));
    let (ctx, events_loop) = &mut cb.build()?;

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    graphics::set_screen_coordinates(
        ctx,
        graphics::Rect::new(
            0.,
            0.,
            constants::WINDOW_WIDTH,
            constants::WINDOW_HEIGHT,
        ),
    )?;

    let state = &mut MainState::new(ctx);
    event::run(ctx, events_loop, state)
}
