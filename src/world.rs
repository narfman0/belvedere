use cgmath::{Point2, Vector2};
use std::time::Duration;

use crate::entity::{Entity, JumpState};

static IMPULSE_X: f32 = 0.1;
static JUMP_Y: f32 = 5.0;

pub struct World {
    pub player: Entity,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub geometries: Vec<Entity>,
    pub jump_duration: u128,
}

impl World {
    pub fn new() -> World {
        // TODO [levels] level will eventually declare its length and define all its geometries
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
            jump_duration: 0,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        match &self.player.jump_state {
            JumpState::Jumping => {
                self.player.vel.y = JUMP_Y;
                if !self.up {
                    self.player.jump_state = JumpState::Cresting;
                }
                self.jump_duration += delta.as_millis();
                if self.jump_duration >= 500 {
                    self.player.jump_state = JumpState::Cresting;
                    self.jump_duration = 0;
                }
            },
            JumpState::Cresting => {
                self.player.vel.y -= 9.81 * delta.as_secs_f32();
                if self.player.vel.y < -JUMP_Y {
                    self.player.jump_state = JumpState::Falling;
                    self.player.vel.y = -JUMP_Y;
                }
            },
            JumpState::Falling => self.player.vel.y = -JUMP_Y,
            JumpState::CanJump => {
                // the -8 represents half a tile. since we don't do slant geometry, this should hopefully maybe generally get us by
                let mut under_player = Entity::new(Point2{x: self.player.pos.x, y: self.player.pos.y-8.0}, self.player.vel,  self.player.size);
                let mut ground = false;
                for geometry in &self.geometries {
                    if under_player.collides(geometry) {
                        ground = true;
                    }
                }
                if !ground {
                    self.player.jump_state = JumpState::Falling;
                }
            },
        }
        
        for geometry in &self.geometries {
            if self.player.collides(geometry) {
                self.player.pos -= self.player.vel;
                let collides_up = (self.player.pos.y - geometry.pos.y).abs() > (self.player.pos.x - geometry.pos.x).abs();
                if collides_up {
                    self.player.vel.y = 0.0;
                    if self.player.pos.y > geometry.pos.y {
                        self.player.jump_state = JumpState::CanJump;
                    }
                } else {
                    self.player.vel.x = 0.0;
                }
            }
        }
        
        if self.up && self.player.jump_state == JumpState::CanJump {
            self.player.vel.y += JUMP_Y;
            self.player.jump_state = JumpState::Jumping;
            self.jump_duration = 0;
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