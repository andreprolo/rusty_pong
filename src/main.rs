use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_PADDING: f32 = 40.0;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 300.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut na::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir;
    }
}

fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };

    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

fn randomized_color(color: Option<&graphics::Color>) -> graphics::Color {
    let mut rng = thread_rng();

    let mut r = match rng.gen_bool(0.5) {
        true => 1.0,
        false => 0.0,
    };
    let mut g = match rng.gen_bool(0.5) {
        true => 1.0,
        false => 0.0,
    };
    let mut b = match rng.gen_bool(0.5) {
        true => 1.0,
        false => 0.0,
    };

    if r == 0.0 && g == 0.0 && b == 0.0 {
        r = 1.0;
        g = 1.0;
        b = 1.0;
    }

    let mut result = graphics::Color::new(r, g, b, 1.0);

    match color {
        Some(c) => {
            if result == *c {
                result = randomized_color(Some(&c));
            }
        }
        None => {}
    }

    result
}

struct MainState {
    running: bool,
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
    color: graphics::Color,
    pause_text: String,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = na::Vector2::new(BALL_SPEED, BALL_SPEED);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            running: false,
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF + RACKET_PADDING, screen_h_half),
            player_2_pos: na::Point2::new(
                screen_w - RACKET_WIDTH_HALF - RACKET_PADDING,
                screen_h_half,
            ),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel,
            player_1_score: 0,
            player_2_score: 0,
            color: randomized_color(None),
            pause_text: "Press Space to Start Game".to_string(),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);

        if self.running {
            move_racket(&mut self.player_1_pos, KeyCode::W, -PLAYER_SPEED * dt, ctx);
            move_racket(&mut self.player_1_pos, KeyCode::S, PLAYER_SPEED * dt, ctx);
            move_racket(&mut self.player_2_pos, KeyCode::Up, -PLAYER_SPEED * dt, ctx);
            move_racket(
                &mut self.player_2_pos,
                KeyCode::Down,
                PLAYER_SPEED * dt,
                ctx,
            );
            clamp(
                &mut self.player_1_pos.y,
                RACKET_HEIGHT_HALF,
                screen_h - RACKET_HEIGHT_HALF,
            );
            clamp(
                &mut self.player_2_pos.y,
                RACKET_HEIGHT_HALF,
                screen_h - RACKET_HEIGHT_HALF,
            );
            self.ball_pos += self.ball_vel * dt;
            if self.ball_pos.x < 0.0 {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_2_score += 1;
                graphics::set_window_title(
                    ctx,
                    &format!(
                        " Rusty Pong | {} - {}",
                        self.player_1_score, self.player_2_score
                    ),
                );
            }
            if self.ball_pos.x > screen_w {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_1_score += 1;
                graphics::set_window_title(
                    ctx,
                    &format!(
                        " Rusty Pong | {} - {}",
                        self.player_1_score, self.player_2_score
                    ),
                );
            }
            if self.ball_pos.y < BALL_SIZE_HALF {
                self.ball_pos.y = BALL_SIZE_HALF;
                self.ball_vel.y = self.ball_vel.y.abs();
            } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
                self.ball_pos.y = screen_h - BALL_SIZE_HALF;
                self.ball_vel.y = -self.ball_vel.y.abs();
            }
            let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
                < self.player_1_pos.x + RACKET_WIDTH_HALF
                && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
                && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
                && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;
            if intersects_player_1 {
                self.ball_vel.x = self.ball_vel.x.abs() * 1.1;
            }
            let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
                < self.player_2_pos.x + RACKET_WIDTH_HALF
                && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
                && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
                && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;
            if intersects_player_2 {
                self.ball_vel.x = -self.ball_vel.x.abs() * 1.1;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket, self.color)?;

        let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);

        let ball_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball, self.color)?;

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = graphics::Text::new(format!(
            " {}     -     {} ",
            self.player_1_score, self.player_2_score
        ));

        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;

        let (score_text_w, score_text_h) = score_text.dimensions(ctx);

        let mut score_pos = na::Point2::new(screen_w_half, 40.0);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        if !self.running {
            let pause_text = graphics::Text::new(self.pause_text.clone());
            let (pause_text_w, pause_text_h) = pause_text.dimensions(ctx);
            let mut pause_pos = na::Point2::new(screen_w_half, 20.0);
            pause_pos -= na::Vector2::new(pause_text_w as f32 * 0.5, pause_text_h as f32 * 0.5);
            draw_param.dest = pause_pos.into();
            graphics::draw(ctx, &pause_text, draw_param)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _key_code: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match _key_code {
            keyboard::KeyCode::Space => {
                self.running = !self.running;
                self.pause_text = "Press Space to Resume Game".to_string();
            }
            keyboard::KeyCode::C => {
                self.color = randomized_color(Some(&self.color));
            }

            keyboard::KeyCode::Escape => ggez::event::quit(_ctx),
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Pong_0", "Prolo")
        .add_resource_path("assets")
        .build()?;

    graphics::set_window_title(&mut ctx, " Rusty Pong | 0 - 0");

    let icon = std::path::Path::new("/ping-pong.png");
    graphics::set_window_icon(&mut ctx, Some(icon))?;

    let mut state = MainState::new(&mut ctx);

    event::run(&mut ctx, &mut event_loop, &mut state)?;

    Ok(())
}
