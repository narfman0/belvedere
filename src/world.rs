use cgmath::{Point2, Vector2};

static IMPULSE_X: f32 = 0.1;
static JUMP_Y: f32 = 1.0;

pub struct World {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl World {
    pub fn new() -> World {
        World{
            pos: Point2::new(10.0, 10.0),
            vel: Vector2::new(0.0, 0.0),
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    pub fn update(&mut self, fps: f32) {
        // TODO change this to more manual control of jump height as player holds jump
        self.vel.y -= 9.81 / fps;
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
    }
}