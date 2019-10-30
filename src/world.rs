use cgmath::{Point2, Vector2};

use crate::entity::Entity;

static IMPULSE_X: f32 = 0.1;
static JUMP_Y: f32 = 1.0;

pub struct World {
    pub player: Entity,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl World {
    pub fn new() -> World {
        World{
            player: Entity::new(Point2::<f32>::new(10.0, 10.0), Vector2::new(0.0, 0.0), 4.0),
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    pub fn update(&mut self, fps: f32) {
        // TODO change this to more manual control of jump height as player holds jump
        self.player.vel.y -= 9.81 / fps;
        // TODO detect collisions/standing state
        if self.player.pos.y < 0.0 {
            self.player.pos.y = 0.0;
            if self.player.vel.y < -0.1 {
                self.player.vel.y = 0.0;
            }
        }
        if self.up {
            self.player.vel.y += JUMP_Y;
        }
        if self.down {
            self.player.vel.y -= JUMP_Y;
        }
        if self.left {
            self.player.vel.x -= IMPULSE_X;
        }
        if self.right {
            self.player.vel.x += IMPULSE_X;
        }
        self.player.pos += self.player.vel;
    }
}