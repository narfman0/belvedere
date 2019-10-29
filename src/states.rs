use cgmath;
use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use cgmath::{Point2, Vector2};

static PLAYER_WIDTH: i32 = 8;
static IMPULSE_X: f32 = 0.1;
static JUMP_Y: f32 = 1.0;

pub struct MainState {
    frames: usize,
    pos: Point2<f32>,
    vel: Vector2<f32>,
    player_image: graphics::Image,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos = Point2::new(10.0, 10.0);
        let vel = Vector2::new(0.0, 0.0);
        let player_image = graphics::Image::new(ctx, "/red.png")?;
        let s = MainState { frames: 0, pos, vel, player_image, up: false, down: false, left: false, right: false };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        self.vel.y -= 9.81 / fps as f32;
        // TODO detect collisions/standing state
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            if self.vel.y < -0.1 {
                self.vel.y = 0.0;
            }
        }
        if self.up {
            self.vel.y += JUMP_Y;
        }
        if self.down {
            self.vel.y -= JUMP_Y;
        }
        if self.left {
            self.vel.x -= IMPULSE_X;
        }
        if self.right {
            self.vel.x += IMPULSE_X;
        }
        self.pos += self.vel;
        self.frames += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let r = ggez::graphics::screen_coordinates(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let player_pos = cgmath::Point2::new(self.pos.x - PLAYER_WIDTH as f32/2.0, r.h - self.pos.y - PLAYER_WIDTH as f32);
        graphics::draw(ctx, &self.player_image, (player_pos,))?;
        graphics::present(ctx)?;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.up = true,
            KeyCode::Down | KeyCode::S => self.down = true,
            KeyCode::Left | KeyCode::A => self.left = true,
            KeyCode::Right | KeyCode::D => self.right = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.up = false,
            KeyCode::Down | KeyCode::S => self.down = false,
            KeyCode::Left | KeyCode::A => self.left = false,
            KeyCode::Right | KeyCode::D => self.right = false,
            _ => (),
        }
    }
}