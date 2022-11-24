use std::fs;

use anyhow::{anyhow, Result};
use platform_dirs::UserDirs;
use raylib::{prelude::{Sound}, RaylibHandle, ffi::PlaySoundMulti};
use serde::{Deserialize, Serialize};

use crate::{
    collision::instances::BOXES,
    map::{image::GameImage, instances::{CURRENT_LEVEL, DIM_LEVEL}},
    player::data::Player, StartupMode,
};

use super::{
    asset::LevelAsset,
    instances::{LEVELS, LEVELS_LOAD_DAY, STARTUP_MODE, LEVELS_LOAD_NIGHT, LEVELS_LOAD_EYE},
};
#[derive(Serialize, Deserialize)]
pub struct Levels {
    levels: Vec<Level>,
}

impl Levels {
    pub fn load(player: &mut Player, rl: &mut RaylibHandle) -> Result<()> {
        {
            let mut levels = match LEVELS.lock() {
                Ok(l) => l,
                Err(_) => return Err(anyhow!("")),
            };

            let curr_s = match STARTUP_MODE.lock() {
                Ok(s) => s,
                Err(_) => return Err(anyhow!("ive stopped caring about these errors"))
            };

            let lv_str: Self = match curr_s.clone() {
                StartupMode::Day => serde_yaml::from_str(LEVELS_LOAD_DAY)?,
                StartupMode::Night => serde_yaml::from_str(LEVELS_LOAD_NIGHT)?,
                StartupMode::Eye => serde_yaml::from_str(LEVELS_LOAD_EYE)?,
                StartupMode::Locked => unreachable!()
            };

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

        Level::load("start", player, rl)?;

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
    pub fn load(name: &str, player: &mut Player, _drawing: &RaylibHandle) -> Result<()> {
        println!("Loading level!");
        let levels = match LEVELS.lock() {
            Ok(l) => l,
            Err(_) => return Err(anyhow!("Failed to lock LEVELS mutex!")),
        };

        println!("Loading...");

        let levels_iter = levels.iter();
        for level in levels_iter {
            if level.id == name {
                let mut dim = match DIM_LEVEL.lock() {
                    Ok(d) => d,
                    Err(_) => return Err(anyhow!("pls"))
                };

                *dim = 1.;
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

                if &level.bg == "cls_eye.png" {
                    let user_dirs = match UserDirs::new() {
                        Some(a) => a,
                        None => return Err(anyhow!("oopy"))
                    };

                    let desktop = user_dirs.desktop_dir;

                    fs::create_dir_all(desktop.as_path())?;

                    let desktop = desktop.join("124.txt");

                    fs::write(desktop, "jump")?;
                    fs::write("glitch.wav", include_bytes!("glitch.wav"))?;
                    
                    let s = match Sound::load_sound("glitch.wav") {
                        Ok(s) => s,
                        Err(_) => return Err(anyhow!("saasdad"))
                    }.to_raw();

                    unsafe {
                        PlaySoundMulti(s);
                    }

                    fs::remove_file("glitch.wav")?;
                }

                break;
            }
        }

        Ok(())
    }
}

