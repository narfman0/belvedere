use cgmath::Point2;
use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

static PLAYER_WIDTH: i32 = 8;
static TILE_WIDTH: i32 = 16;
static PLAYER_WIDTH_F: f32 = PLAYER_WIDTH as f32;
static TILE_WIDTH_F: f32 = TILE_WIDTH as f32;

pub struct LevelState {
    frames: usize,
    player_image: graphics::Image,
    green_tile: graphics::Image,
    world: crate::world::World,
}

impl LevelState {
    pub fn new(ctx: &mut Context) -> GameResult<LevelState> {
        let world = crate::world::World::new();
        let player_image = graphics::Image::new(ctx, "/red.png")?;
        let green_tile = graphics::Image::new(ctx, "/greentile.png")?;
        let s = LevelState { frames: 0, player_image, green_tile, world };
        Ok(s)
    }
}

impl event::EventHandler for LevelState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        self.world.update(fps as f32);
        self.frames += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let r = ggez::graphics::screen_coordinates(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        for geometry in &self.world.geometries {
            let pos = Point2::new(geometry.pos.x as f32 - TILE_WIDTH_F/2.0,
                r.h - geometry.pos.y as f32 - TILE_WIDTH_F/2.0);
            graphics::draw(ctx, &self.green_tile, (pos,))?;
        }
        let player_pos = Point2::new(self.world.player.pos.x - PLAYER_WIDTH_F/2.0,
            r.h - self.world.player.pos.y - PLAYER_WIDTH_F/2.0);
        graphics::draw(ctx, &self.player_image, (player_pos,))?;
        graphics::present(ctx)?;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.world.up = true,
            KeyCode::Down | KeyCode::S => self.world.down = true,
            KeyCode::Left | KeyCode::A => self.world.left = true,
            KeyCode::Right | KeyCode::D => self.world.right = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::W => self.world.up = false,
            KeyCode::Down | KeyCode::S => self.world.down = false,
            KeyCode::Left | KeyCode::A => self.world.left = false,
            KeyCode::Right | KeyCode::D => self.world.right = false,
            _ => (),
        }
    }
}