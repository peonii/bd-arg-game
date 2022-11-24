#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]

use std::{time::Instant, process, fs};

use anyhow::{anyhow, Result};
use map::{instances::{CURRENT_LEVEL, DIM_LEVEL, TIMER_START, STARTUP_MODE}, level::Levels};
use platform_dirs::AppDirs;
use raylib::{
    prelude::{Color, RaylibDraw},
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread, ffi::InitAudioDevice,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StartupMode {
    Day,
    Night,
    Eye,
    Locked
}

mod collision;
mod map;
mod player;

pub use player::instance::PLAYER;
use rust_embed::EmbeddedFile;
use serde::{Serialize, Deserialize};

/// Init function for the game.
///
/// # Errors
/// - When failing to lock mutex
/// - When failing to load levels (due to incorrect parsing of the .yml file)
pub fn init(rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<()> {
    unsafe {
        InitAudioDevice();
    }

    let mut player = match PLAYER.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")),
    };

    let app_dirs = match AppDirs::new(Some("arggame"), true) {
        Some(a) => a,
        None => return Err(anyhow!("oopy"))
    };

    let cache_path = app_dirs.cache_dir;

    fs::create_dir_all(cache_path.as_path())?;

    let cache_path = cache_path.join("data.yml");

    let startup_mode_str = fs::read_to_string(cache_path.as_path()).unwrap_or(String::new());

    let startup_mode: StartupMode = match serde_yaml::from_str(startup_mode_str.as_str()) {
        Ok(s) => s,
        Err(_) => StartupMode::Day
    };

    {
        let mut curr_s = match STARTUP_MODE.lock() {
            Ok(s) => s,
            Err(_) => return Err(anyhow!("ive stopped caring about these errors"))
        };

        *curr_s = startup_mode;

        match curr_s.clone() {
            StartupMode::Locked => process::exit(0),
            _ => {}
        }
    }

    Levels::load(&mut player, rl)?;
    rl.set_target_fps(60);

    Ok(())
}

/// Main update function that is called every tick.
///
/// # Errors
/// - When failing to lock one of the many mutexes
/// - When failing to load background (happens due to file not existing)
/// - When failing to perform any logic
pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
    let now = Instant::now();

    if now.duration_since(*TIMER_START).as_secs() > 300 {
        let curr_s = match STARTUP_MODE.lock() {
            Ok(s) => s,
            Err(_) => return Err(anyhow!("ive stopped caring about these errors"))
        };

        let new_s = match curr_s.clone() {
            StartupMode::Day => StartupMode::Night,
            StartupMode::Night => StartupMode::Eye,
            StartupMode::Eye => StartupMode::Locked,
            StartupMode::Locked => unreachable!()
        };

        let app_dirs = match AppDirs::new(Some("arggame"), true) {
            Some(a) => a,
            None => return Err(anyhow!("oopy"))
        };

        let cache_path = app_dirs.cache_dir.join("data.yml");

        let startup_mode_str = serde_yaml::to_string(&new_s)?;

        fs::write(cache_path, startup_mode_str)?;

        process::exit(0);
    }

    let tex: Texture2D;
    {
        let bg = match CURRENT_LEVEL.lock() {
            Ok(p) => p,
            Err(_) => return Err(anyhow!("Error locking CURRENT_LEVEL mutex!")), // please format your errors like this thank you
        };

        let bg_ref = bg.as_ref();
        let actual_file: &EmbeddedFile = match bg_ref {
            Some(e) => e,
            None => return Err(anyhow!("Failed to load bg!")),
        }; // we CAN do this as we are 100% sure this is a correct file
           //
        let data_size = actual_file.data.len().try_into()?;

        let img = match Image::load_image_from_mem(
            ".png",
            &Vec::from(actual_file.data.clone()),
            data_size,
        ) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to load image!")),
        };
        tex = match rl.load_texture_from_image(thread, &img) {
            Ok(t) => t,
            Err(_) => return Err(anyhow!("Failed to load texture from image!")),
        };
    }
    {
        let mut player = match PLAYER.lock() {
            Ok(p) => p,
            Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")), // please format your errors like this thank you
        };
        
        player.load_text(rl, thread)?;
    }
    let mut drawing = rl.begin_drawing(thread);

    // if we don't do this we get epileptic flashing which isn't cute at all
    drawing.clear_background(Color::WHITE);
    {

        let mut dim = match DIM_LEVEL.lock() {
            Ok(d) => d,
            Err(_) => return Err(anyhow!("pls"))
        };

        let color = Color::color_from_hsv(0., 0., 1. - *dim);

        if *dim > 0. {
            *dim -= 0.01;
        } else {
            *dim = 0.;
        }

        println!("{}", *dim);

        drawing.draw_texture(&tex, 0, 0, color);
    }
    drawing.draw_fps(10, 10);

    {
        // this is to ensure "thread safety" (we're running the game on one thread :steamhappy:)
        let mut player = match PLAYER.lock() {
            Ok(p) => p,
            Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")), // please format your errors like this thank you
        };

        player.update(&mut drawing)?;
    }


    Ok(())
}
