use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;

use ggez::Context;

use std::time::Duration;

use rand::{thread_rng, Rng, RngCore};

use super::gameoverstate::{GameOverState, Winner};
use super::shapes::{Ball, GameObject, Paddle};
use super::utils;

use crate::constants::{FONT_SIZE, VIRTUAL_WORLD_HEIGHT, VIRTUAL_WORLD_WIDTH};
use crate::traits::{State, StateWithTransition, Transition};

const BALL_RADIUS: f32 = 4.;
const PADDLE_HEIGHT: f32 = 30.;
const PADDLE_WIDTH: f32 = 6.;

pub struct GameState<'a> {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    player_one_score: i32,
    player_two_score: i32,
    rng: Box<dyn RngCore + 'a>,
    next_round: bool,
}

impl StateWithTransition for GameState<'_> {}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let player_one_serves: bool = rng.gen();

        let ball_d_x = rng.gen_range(140., 200.);
        let ball_d_x: f32 = ball_d_x * if player_one_serves { 1. } else { -1. };
        let ball_d_y: f32 = rng.gen_range(-50., 50.);

        GameState {
            ball: Ball::new(
                VIRTUAL_WORLD_WIDTH / 2. - BALL_RADIUS / 2.,
                VIRTUAL_WORLD_HEIGHT / 2. - BALL_RADIUS / 2.,
                ball_d_x,
                ball_d_y,
                BALL_RADIUS,
            ),
            player1: Paddle::new(
                10.,
                VIRTUAL_WORLD_HEIGHT / 2. - PADDLE_HEIGHT / 2.,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
                KeyCode::W,
                KeyCode::S,
            ),
            player2: Paddle::new(
                VIRTUAL_WORLD_WIDTH - PADDLE_WIDTH - 10.,
                VIRTUAL_WORLD_HEIGHT / 2. - PADDLE_HEIGHT / 2.,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
                KeyCode::Up,
                KeyCode::Down,
            ),
            player_one_score: 0,
            player_two_score: 0,
            next_round: false,
            rng: Box::new(rng),
        }
    }
}

impl State for GameState<'_> {
    fn update(&mut self, ctx: &Context, dt: Duration) {
        self.ball.update(ctx, dt);

        let world_rect = Rect::new(0., 0., VIRTUAL_WORLD_WIDTH, VIRTUAL_WORLD_HEIGHT);
        if let Some(side) = collides_with_boundaries(&mut self.ball, world_rect) {
            match side {
                Side::Bottom => {
                    self.ball.d_y = -self.ball.d_y;
                    self.ball.y = VIRTUAL_WORLD_HEIGHT - self.ball.radius * 2.;
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
        if collides_with_paddle(&self.ball, &self.player1) {
            self.ball.d_x = -self.ball.d_x * 1.03;
            self.ball.x = self.player1.x + self.player1.width + self.ball.radius;
            self.ball.d_y = if self.ball.d_y > 0. {
                self.rng.gen_range(10., 150.)
            } else {
                -self.rng.gen_range(10., 150.)
            };
        }

        if collides_with_paddle(&self.ball, &self.player2) {
            self.ball.d_x = -self.ball.d_x * 1.03;
            self.ball.x = self.player2.x - self.player1.width - self.ball.radius;

            self.ball.d_y = if self.ball.d_y > 0. {
                self.rng.gen_range(10., 150.)
            } else {
                -self.rng.gen_range(10., 150.)
            };
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

impl<'a> Transition for GameState<'a> {
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
        Point2::new(VIRTUAL_WORLD_WIDTH / 3., VIRTUAL_WORLD_HEIGHT / 3.),
        FONT_SIZE,
    );
    utils::draw_text_at_location(
        ctx,
        format!("{}", player_two_score),
        Point2::new(VIRTUAL_WORLD_WIDTH * 2. / 3., VIRTUAL_WORLD_HEIGHT / 3.),
        FONT_SIZE,
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
