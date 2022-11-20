use anyhow::{Result, anyhow};
use map::{level::Levels, instances::CURRENT_LEVEL};
use raylib::{RaylibHandle, RaylibThread, prelude::{Color, RaylibDraw}, texture::Image};

mod player;
mod collision;
mod map;

pub use player::instance::PLAYER;
use rust_embed::EmbeddedFile;

pub fn init(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<()> {
    let mut player = match PLAYER.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking PLAYER mutex!"))
    };

    Levels::load(&mut player)?;

    Ok(())
}

pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
    let bg = match CURRENT_LEVEL.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking CURRENT_LEVEL mutex!")) // please format your errors like this thank you
    };

    let actual_file: &EmbeddedFile = bg.as_ref().unwrap(); // we CAN do this as we are 100% sure this is a correct file

    let img = match Image::load_image_from_mem(".png", &Vec::from(actual_file.data.clone()), actual_file.data.len().try_into().unwrap()) {
        Ok(i) => i,
        Err(_) => return Err(anyhow!("Failed to load image!"))
    };
    let tex = rl.load_texture_from_image(&thread, &img).expect("okay whatever");
    let mut drawing = rl.begin_drawing(thread);

    // if we don't do this we get epileptic flashing which isn't cute at all
    drawing.clear_background(Color::WHITE);

    drawing.draw_texture(&tex, 0, 0, Color::WHITE);
    

    {
        // this is to ensure "thread safety" (we're running the game on one thread :steamhappy:)
        let mut player = match PLAYER.lock() {
            Ok(p) => p,
            Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")) // please format your errors like this thank you
        };

        player.update(&mut drawing)?;
    }

    Ok(())
}