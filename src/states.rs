use cgmath;
use ggez;
use nphysics2d;

use ggez::nalgebra::Vector2 as v2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use cgmath::{Point2, Vector2};

static PLAYER_WIDTH: i32 = 4;

pub struct MainState {
    frames: usize,
    pos: Point2<f32>,
    vel: Vector2<f32>,
    player_image: graphics::Image,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos = Point2::new(0.0, 0.0);
        let vel = Vector2::new(0.0, 0.0);
        let player_image = graphics::Image::new(ctx, "/red.png")?;

        let mechanical_world = DefaultMechanicalWorld::new(v2::new(0.0, -9.81));
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        let s = MainState { frames: 0, pos, vel, player_image, up: false, down: false, left: false, right: false,
            mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.up {
            self.vel.y += 0.1;
        }
        if self.down {
            self.vel.y -= 0.1;
        }
        if self.left {
            self.vel.x -= 0.1;
        }
        if self.right {
            self.vel.x += 0.1;
        }
        self.pos += self.vel;

        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        self.frames += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let r = ggez::graphics::screen_coordinates(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let player_pos = cgmath::Point2::new(self.pos.x - PLAYER_WIDTH as f32/2.0, r.h - (self.pos.y - PLAYER_WIDTH as f32/2.0));
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