use cgmath;
use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use cgmath::{Point2, Vector2};

pub struct MainState {
    frames: usize,
    pos: Point2<i32>,
    vel: Vector2<i32>,
    text: graphics::Text,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new(("A", font, 48.0));
        let pos = Point2::new(0, 0);
        let vel = Vector2::new(0, 0);
        let s = MainState { frames: 0, pos, vel, text };
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
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let dest_point = cgmath::Point2::new(self.pos.x as f32, self.pos.y as f32);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.vel.y += 10,
            KeyCode::Left | KeyCode::A => self.vel.x -= 1,
            KeyCode::Right | KeyCode::D => self.vel.x += 1,
            _ => (),
        }
    }
}