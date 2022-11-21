use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    collision::instances::BOXES,
    map::{image::GameImage, instances::CURRENT_LEVEL},
    player::data::Player,
};

use super::{
    asset::LevelAsset,
    instances::{LEVELS, LEVELS_LOAD},
};
#[derive(Serialize, Deserialize)]
pub struct Levels {
    levels: Vec<Level>,
}

impl Levels {
    pub fn load(player: &mut Player) -> Result<()> {
        {
            let mut levels = match LEVELS.lock() {
                Ok(l) => l,
                Err(_) => return Err(anyhow!("")),
            };

            let lv_str: Self = serde_yaml::from_str(LEVELS_LOAD)?;

            /*
            for level in lv_str.levels.iter_mut() {
                level.bg_bytes = match GameImage::get(&level.bg) {
                    Some(i) => Some(Box::new(i)),
                    None => return Err(anyhow!("Error getting background!")),
                };
            }
            */

            *levels = lv_str.levels;
        }

        Level::load("start", player)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct LevelCoords {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    id: String,
    bg: String,
    starting_coords: LevelCoords,
    assets: Vec<LevelAsset>,
}

impl Level {
    pub fn load(name: &str, player: &mut Player) -> Result<()> {
        println!("Loading level!");
        let levels = match LEVELS.lock() {
            Ok(l) => l,
            Err(_) => return Err(anyhow!("Failed to lock LEVELS mutex!")),
        };

        println!("Loading...");

        let levels_iter = levels.iter();
        for level in levels_iter {
            if level.id == name {
                {
                    let mut boxes = match BOXES.lock() {
                        Ok(b) => b,
                        Err(_) => return Err(anyhow!("Error locking BOXES mutex")),
                    };

                    *boxes = vec![];
                }

                for asset in &level.assets {
                    asset.load()?;
                }

                let mut current = match CURRENT_LEVEL.lock() {
                    Ok(l) => l,
                    Err(_) => return Err(anyhow!("Error locking CURRENT_LEVEL mutex")),
                };

                *current = match GameImage::get(&level.bg) {
                    Some(i) => Some(Box::new(i)),
                    None => return Err(anyhow!("Error getting background!")),
                };

                player.move_to(level.starting_coords.x, level.starting_coords.y);

                break;
            }
        }

        Ok(())
    }
}

