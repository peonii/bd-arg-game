use anyhow::{Result, anyhow};
use raylib::prelude::*;


use super::instances::BOXES;

#[derive(Clone)]
pub struct CollisionBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl CollisionBox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x, y, width, height
        }
    }

    pub fn register(cb: CollisionBox) -> Result<()> {
        let mut boxes = match BOXES.lock() {
            Ok(b) => b,
            Err(_) => return Err(anyhow!("Error locking BOXES mutex"))
        };

        boxes.push(cb.clone());

        Ok(())
    }

    pub fn collides_with(&self, x: i32, y: i32) -> bool {
        if (x > self.x && x < (self.x + self.width)) && 
            (y > self.y && y < (self.y + self.height)) {
            return true;
        }

        false
    }

    pub fn draw(&self, drawing: &mut RaylibDrawHandle) {
        drawing.draw_rectangle(self.x, self.y, self.width, self.height, Color::BLACK);
    }
}