use ggez;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics;

use ggez::graphics::BLACK;

use ggez::timer;
use ggez::{Context, GameResult};

use std::path;

mod traits;
use traits::StateWithTransition;

mod states;
use states::startscreenstate::StartScreenState;

mod constants;

struct MainState {
    state: Box<dyn StateWithTransition>,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            state: Box::new(StartScreenState { ready: false }),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::delta(ctx);
        self.state.update(ctx, dt);
        if let Some(new_state) = self.state.transition() {
            self.state = new_state
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);
        self.state.draw(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let mut window_mode = WindowMode::default();
    window_mode.width = constants::WORLD_WIDTH;
    window_mode.height = constants::WORLD_HEIGHT;
    let cb = ggez::ContextBuilder::new("Pong", "Ivan Zub (zub.ivan@gmail.com)")
        .window_mode(window_mode)
        .add_resource_path(path::PathBuf::from("./assets"));
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new();
    event::run(ctx, events_loop, state)
}
