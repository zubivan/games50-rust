use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;

use ggez::Context;

use std::time::Duration;

use rand::{thread_rng, Rng};

use super::gameoverstate::{GameOverState, Winner};
use super::shapes::{Ball, GameObject, Paddle};
use super::utils;

use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

pub struct GameState {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    player_one_score: i32,
    player_two_score: i32,
    next_round: bool,
}

impl StateWithTransition for GameState {}

impl GameState {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let player_one_serves: bool = rng.gen();
        let ball_d_x: f32 = if player_one_serves {
            rng.gen_range(250., 350.)
        } else {
            -rng.gen_range(250., 350.)
        };
        let ball_d_y: f32 = rng.gen_range(-300., 300.);

        GameState {
            ball: Ball::new(
                WORLD_WIDTH / 2. - 5.,
                WORLD_HEIGHT / 2. - 5.,
                ball_d_x,
                ball_d_y,
                10.,
            ),
            player1: Paddle::new(
                10.,
                WORLD_HEIGHT / 2. - 50.,
                20.,
                100.,
                KeyCode::W,
                KeyCode::S,
            ),
            player2: Paddle::new(
                WORLD_WIDTH - 30.,
                WORLD_HEIGHT / 2. - 50.,
                20.,
                100.,
                KeyCode::Up,
                KeyCode::Down,
            ),
            player_one_score: 0,
            player_two_score: 0,
            next_round: false,
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &Context, dt: Duration) {
        self.ball.update(ctx, dt);

        let world_rect = Rect::new(0., 0., WORLD_WIDTH, WORLD_HEIGHT);
        if let Some(side) = collides_with_boundaries(&mut self.ball, world_rect) {
            match side {
                Side::Bottom => {
                    self.ball.d_y = -self.ball.d_y;
                    self.ball.y = WORLD_HEIGHT - self.ball.radius * 2.;
                }
                Side::Top => {
                    self.ball.d_y = -self.ball.d_y;
                    self.ball.y = self.ball.radius * 2.;
                }
                Side::Left => {
                    self.player_two_score += 1;
                    self.next_round = true;
                }
                Side::Right => {
                    self.player_one_score += 1;
                    self.next_round = true;
                }
            }
        };

        let mut rng = thread_rng();
        if collides_with_paddle(&self.ball, &self.player1) {
            self.ball.d_x = -self.ball.d_x * 1.03;
            self.ball.d_y = if self.ball.d_y > 0. {
                rng.gen_range(10., 300.)
            } else {
                -rng.gen_range(10., 300.)
            };
            self.ball.x = self.player1.x + self.player1.width + self.ball.radius;
        }

        if collides_with_paddle(&self.ball, &self.player2) {
            self.ball.d_x = -self.ball.d_x * 1.03;
            self.ball.d_y = if self.ball.d_y > 0. {
                rng.gen_range(10., 300.)
            } else {
                -rng.gen_range(10., 300.)
            };
            self.ball.x = self.player2.x - self.player1.width;
        }

        self.player1.update(ctx, dt);
        self.player2.update(ctx, dt);
    }
    fn draw(&self, ctx: &mut Context) {
        self.ball.draw(ctx);
        self.player1.draw(ctx);
        self.player2.draw(ctx);
        draw_score(ctx, self.player_one_score, self.player_two_score);
    }
}

impl Transition for GameState {
    fn transition(&self) -> Option<Box<dyn StateWithTransition>> {
        let player_one_won = self.player_one_score == 10;
        let player_two_won = self.player_two_score == 10;
        if player_one_won || player_two_won {
            Some(Box::new(GameOverState::new(if player_one_won {
                Winner::PlayerOne
            } else {
                Winner::PlayerTwo
            })))
        } else if self.next_round {
            let mut next_state = GameState::new();
            next_state.player_one_score = self.player_one_score;
            next_state.player_two_score = self.player_two_score;
            Some(Box::new(next_state))
        } else {
            None
        }
    }
}

enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

fn draw_score(ctx: &mut Context, player_one_score: i32, player_two_score: i32) {
    utils::draw_text_at_location(
        ctx,
        format!("{}", player_one_score),
        Point2::new(WORLD_WIDTH / 3., WORLD_HEIGHT / 3.),
        120.,
    );
    utils::draw_text_at_location(
        ctx,
        format!("{}", player_two_score),
        Point2::new(WORLD_WIDTH * 2. / 3., WORLD_HEIGHT / 3.),
        120.,
    );
}

fn collides_with_boundaries(ball: &mut Ball, boundaries: Rect) -> Option<Side> {
    if ball.y < boundaries.y + ball.radius {
        Some(Side::Top)
    } else if ball.y > boundaries.h - ball.radius {
        Some(Side::Bottom)
    } else if ball.x < boundaries.x + ball.radius {
        Some(Side::Left)
    } else if ball.x > boundaries.w + ball.radius {
        Some(Side::Right)
    } else {
        None
    }
}

fn collides_with_paddle(ball: &Ball, paddle: &Paddle) -> bool {
    let ball_rect = Rect::new(ball.x, ball.y, ball.radius * 2., ball.radius * 2.);
    let paddle_rect = Rect::new(paddle.x, paddle.y, paddle.width, paddle.heigh);
    ball_rect.overlaps(&paddle_rect)
}
