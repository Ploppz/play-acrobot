// use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics::*, Context, GameResult};
use ggez::event::Keycode;
use std::time::{Duration, Instant};
use rsrl::{
    domains::{Domain, Observation, Acrobat},
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
    env: Acrobat,
    action: usize,
    last_update: Instant,
}
impl Game {
    pub fn new() -> Game {
        Game {
            env: Acrobat::default(),
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
        clear(ctx);
        let state = self.env.emit();
        let state = state.state();
        let (theta1, theta2) = (state[0] as f32, state[1] as f32);

        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        // graphics::rectangle(ctx, color, graphics::DrawMode::fill(), self.pos.into())

        
        // set_color(ctx, [0.5, 0.5, 0.5, 1.0].into())?;
        let pos1 = Point2::new(WIDTH*0.5, HEIGHT*0.5);
        let rect1 = Mesh::new_polygon(ctx, DrawMode::Fill, &rectangle(m(0.1), m(1.0)))?;
        draw(ctx, &rect1, pos1, theta1)?;


        let pos2 = Point2::new(pos1.x - m(1.0)*theta1.sin(), pos1.y + m(1.0)*theta1.cos());
        let rect2 = Mesh::new_polygon(ctx, DrawMode::Fill, &rectangle(m(0.1), m(1.0)))?;
        draw(ctx, &rect2, pos2, theta1 + theta2)?;

        present(ctx);
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
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

