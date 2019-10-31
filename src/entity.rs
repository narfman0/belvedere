use cgmath::{Point2, Vector2};

pub struct Entity {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub size: Vector2<f32>,
    pub grounded: bool,
}

impl Entity {
    pub fn new(pos: Point2<f32>, vel: Vector2<f32>, size: Vector2<f32>) -> Entity {
        Entity{pos, vel, size, grounded: false}
    }

    #[allow(dead_code)]
    pub fn collides(&mut self, entity: &Entity) -> bool {
        return self.pos.x - self.size.x/2.0 < entity.pos.x + entity.size.x/2.0 &&
            self.pos.x + self.size.x/2.0 > entity.pos.x - entity.size.y/2.0 &&
            self.pos.y - self.size.y/2.0 < entity.pos.y + entity.size.y/2.0 &&
            self.pos.y + self.size.y/2.0 > entity.pos.y - entity.size.y/2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collides_side() {
        let mut e1 = Entity::new(Point2::<f32>::new(10.0, 0.0), Vector2::new(0.0, 0.0),  Vector2::new(8.0, 8.0));
        let mut e2 = Entity::new(Point2::<f32>::new(8.0, 0.0), Vector2::new(0.0, 0.0),  Vector2::new(8.0, 8.0));
        assert!(e1.collides(&e2));
        e2.pos.x = -2.0;
        assert!(!e1.collides(&e2));
    }
}