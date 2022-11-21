use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::collision::collbox::CollisionBox;

#[derive(Serialize, Deserialize)]
pub struct LevelAsset {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    connector_to: String,
}

impl LevelAsset {
    pub fn load(&self) -> Result<()> {
        CollisionBox::new(
            self.x,
            self.y,
            self.width,
            self.height,
            self.connector_to.clone(),
        )
        .register()?;

        Ok(())
    }
}

