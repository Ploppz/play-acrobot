// use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics::*, Context, GameResult};
use ggez::event::Keycode;
use std::time::{Duration, Instant};
use rsrl::{
    domains::{Domain, Observation, Acrobot},
    core::Controller,
};

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;
const SCREEN_SIZE: (u32, u32) = (WIDTH as u32, HEIGHT as u32);
const UPDATES_PER_SECOND: f32 = 20.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("Acrobot", "Ploppz")
        .window_setup(ggez::conf::WindowSetup::default().title("Acrobot!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build().expect("Failed to build ggez context");

    let state = &mut Game::new();
    event::run(ctx, state);
}

struct Game {
    env: Acrobot,
    action: usize,
    last_update: Instant,
}
impl Game {
    pub fn new() -> Game {
        Game {
            env: Acrobot::default(),
            action: 1,
            last_update: Instant::now(),
        }
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            self.env.step(self.action);
            println!("Action: {}", self.action);
            self.last_update = Instant::now();
        }
        // Finally we return `Ok` to indicate we didn't run into any errors
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.env.render(ctx);
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => self.action = 2,
            Keycode::Right => self.action = 0,
            Keycode::Escape => std::process::exit(0),
            _ => {},
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => if self.action == 2 {self.action = 1},
            Keycode::Right => if self.action == 0 {self.action = 1},
            _ => {},
        }
    }
}

fn rectangle(w: f32, h: f32) -> Vec<Point2> {
    vec![Point2::new(-w*0.5, 0.0),
    Point2::new(w*0.5, 0.0),
    Point2::new(w*0.5, h),
    Point2::new(-w*0.5, h),
    Point2::new(-w*0.5, 0.0)]
}
/// Convert from meters to pixels
fn m(x: f32) -> f32 {
    x * 70.0
}

