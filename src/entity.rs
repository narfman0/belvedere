use cgmath::{Point2, Vector2};

pub struct Entity {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub size: f32,
}

impl Entity {
    pub fn new(pos: Point2<f32>, vel: Vector2<f32>, size: f32) -> Entity {
        Entity{pos, vel, size}
    }

    pub fn collides(&mut self, entity: Entity) -> bool {
        return self.pos.x < entity.pos.x + entity.size &&
            self.pos.x + self.size > entity.pos.x &&
            self.pos.y < entity.pos.y + entity.size &&
            self.pos.y + self.size > entity.pos.y;
    }
}