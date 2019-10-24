use cgmath;
use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use cgmath::{Point2, Vector2};

static PLAYER_WIDTH: i32 = 4;

pub struct MainState {
    frames: usize,
    pos: Point2<i32>,
    vel: Vector2<i32>,
    player_image: graphics::Image,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos = Point2::new(0, 0);
        let vel = Vector2::new(0, 0);
        let player_image = graphics::Image::new(ctx, "/red.png")?;
        let s = MainState { frames: 0, pos, vel, player_image };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos += self.vel;
        self.frames += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let r = ggez::graphics::screen_coordinates(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let player_pos = cgmath::Point2::new((self.pos.x - PLAYER_WIDTH/2) as f32, r.h - (self.pos.y - PLAYER_WIDTH/2) as f32);
        graphics::draw(ctx, &self.player_image, (player_pos,))?;
        graphics::present(ctx)?;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.vel.y += 1,
            KeyCode::Down | KeyCode::S => self.vel.y -= 1,
            KeyCode::Left | KeyCode::A => self.vel.x -= 1,
            KeyCode::Right | KeyCode::D => self.vel.x += 1,
            _ => (),
        }
    }
}