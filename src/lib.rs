#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]

use anyhow::{anyhow, Result};
use map::{instances::CURRENT_LEVEL, level::Levels};
use raylib::{
    prelude::{Color, RaylibDraw},
    texture::Image,
    RaylibHandle, RaylibThread,
};

mod collision;
mod map;
mod player;

pub use player::instance::PLAYER;
use rust_embed::EmbeddedFile;

/// Init function for the game.
///
/// # Errors
/// - When failing to lock mutex
/// - When failing to load levels (due to incorrect parsing of the .yml file)
pub fn init(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<()> {
    let mut player = match PLAYER.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")),
    };

    Levels::load(&mut player)?;

    Ok(())
}

/// Main update function that is called every tick.
///
/// # Errors
/// - When failing to lock one of the many mutexes
/// - When failing to load background (happens due to file not existing)
/// - When failing to perform any logic
pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
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

    let img =
        match Image::load_image_from_mem(".png", &Vec::from(actual_file.data.clone()), data_size) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to load image!")),
        };
    let tex = match rl.load_texture_from_image(thread, &img) {
        Ok(t) => t,
        Err(_) => return Err(anyhow!("Failed to load texture from image!")),
    };
    let mut drawing = rl.begin_drawing(thread);

    // if we don't do this we get epileptic flashing which isn't cute at all
    drawing.clear_background(Color::WHITE);

    drawing.draw_texture(&tex, 0, 0, Color::WHITE);

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

