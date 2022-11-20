use anyhow::{Result, anyhow};
use rust_embed::EmbeddedFile;
use serde::{Serialize, Deserialize};

use crate::{player::data::Player, collision::instances::BOXES, map::{instances::CURRENT_LEVEL, image::GameImage}};

use super::{asset::LevelAsset, instances::{LEVELS, LEVELS_LOAD}};
#[derive(Serialize, Deserialize)]
pub struct Levels {
    levels: Vec<Level>
}

impl Levels {
    pub fn load(player: &mut Player) -> Result<()> {
        {
            let mut levels = match LEVELS.lock() {
                Ok(l) => l,
                Err(_) => return Err(anyhow!(""))
            };

            let mut lv_str: Self = serde_yaml::from_str(LEVELS_LOAD)?;

            for level in lv_str.levels.iter_mut() {
                level.bg_bytes = match GameImage::get(&level.bg) {
                    Some(i) => Some(Box::new(i)),
                    None => return Err(anyhow!("Error getting background!"))
                };
            }

            *levels = lv_str.levels
        }

        Level::load("start", player)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct LevelCoords {
    x: i32,
    y: i32
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    id: String,
    bg: String,
    starting_coords: LevelCoords,
    assets: Vec<LevelAsset>,

    #[serde(skip)]
    bg_bytes: Option<Box<EmbeddedFile>>
}

impl Level {
    pub fn load(name: &str, player: &mut Player) -> Result<()> {
        println!("Loading level!");
        let levels = match LEVELS.lock() {
            Ok(l) => l,
            Err(_) => return Err(anyhow!("Failed to lock LEVELS mutex!"))
        };

        println!("Loading...");

        for level in levels.iter() {
            if level.id == name {
                {
                    let mut boxes = match BOXES.lock() {
                        Ok(b) => b,
                        Err(_) => return Err(anyhow!("Error locking BOXES mutex"))
                    };

                    *boxes = vec![];
                }

                for asset in &level.assets {
                    asset.load()?;
                }

                let mut current = match CURRENT_LEVEL.lock() {
                    Ok(l) => l,
                    Err(_) => return Err(anyhow!("Failed to lock CURRENT_LEVEL mutex!"))
                };

                *current = &level.bg_bytes;

                player.move_to(level.starting_coords.x, level.starting_coords.y);
            }
        }


        Ok(())
    }
}