use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::collision::collbox::CollisionBox;

#[derive(Serialize, Deserialize)]
pub struct LevelAsset {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    connector_to: String
}

impl LevelAsset {
    pub fn load(&self) -> Result<()> {
        let cb = CollisionBox::new(self.x, self.y, self.width, self.height, self.connector_to.clone());

        CollisionBox::register(cb)?;

        Ok(())
    }
}