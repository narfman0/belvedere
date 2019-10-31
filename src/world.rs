use cgmath::{Point2, Vector2};

use crate::entity::Entity;

static IMPULSE_X: f32 = 0.1;
static JUMP_Y: f32 = 5.0;

pub struct World {
    pub player: Entity,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub geometries: Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        // TODO [levels] level will eventuall declare its length, and define all its geometries
        let mut geometries = Vec::new();
        for x in (0..480).step_by(16) {
            geometries.push(Entity::new(Point2{x: x as f32+8.0, y: 8.0}, Vector2{x: 0.0, y: 0.0}, Vector2{x: 16.0, y: 16.0}));
        }
        World{
            player: Entity::new(Point2{x: 32.0, y: 48.0}, Vector2{x: 0.0, y: 0.0},  Vector2{x: 8.0, y: 8.0}),
            left: false,
            right: false,
            up: false,
            down: false,
            geometries: geometries,
        }
    }

    pub fn update(&mut self, fps: f32) {
        // TODO [jump] change this to more manual control of jump height as player holds jump
        if !self.player.grounded {
            self.player.vel.y -= 9.81 / fps;
        }
        // TODO [physics] handle collisions
        // 1. determine if y or x velocity should be zeroed
        // 2. determine if player is grounded
        self.player.grounded = false;
        for geometry in &self.geometries {
            if self.player.collides(geometry) {
                self.player.pos -= self.player.vel;
                let collides_up = (self.player.pos.y - geometry.pos.y).abs() > (self.player.pos.x - geometry.pos.x).abs();
                if collides_up {
                    self.player.vel.y = 0.0;
                    self.player.grounded = true;
                } else {
                    self.player.vel.x = 0.0;
                }
            }
        }
        // TODO [jump] change this to more manual control of jump height as player holds jump
        if self.up && self.player.grounded {
            self.player.vel.y += JUMP_Y;
        }
        if self.left {
            self.player.vel.x -= IMPULSE_X;
        }
        if self.right {
            self.player.vel.x += IMPULSE_X;
        }
        if !self.left && !self.right {
            self.player.vel.x /= 1.01;
        }
        self.player.pos += self.player.vel;
    }
}