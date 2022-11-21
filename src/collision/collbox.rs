use anyhow::{anyhow, Result};

use super::instances::BOXES;

#[derive(Clone)]
pub struct CollisionBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    pub portal_to: String,
}

impl CollisionBox {
    pub const fn new(x: i32, y: i32, width: i32, height: i32, portal_to: String) -> Self {
        Self {
            x,
            y,
            width,
            height,
            portal_to,
        }
    }

    pub fn register(&self) -> Result<()> {
        let mut boxes = match BOXES.lock() {
            Ok(b) => b,
            Err(_) => return Err(anyhow!("Error locking BOXES mutex")),
        };

        boxes.push(self.clone());

        Ok(())
    }

    pub const fn collides_with(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        if (x + width > self.x && x - width < (self.x + self.width))
            && (y + height > self.y && y - height < (self.y + self.height))
        {
            return true;
        }

        false
    }
}

