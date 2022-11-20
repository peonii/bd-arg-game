use anyhow::{Result, anyhow};
use collision::instances::BOXES;
use map::level::Levels;
use raylib::{RaylibHandle, RaylibThread, prelude::{Color, RaylibDraw}};

mod player;
mod collision;
mod map;

pub use player::instance::PLAYER;

pub fn init(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<()> {
    let mut player = match PLAYER.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking PLAYER mutex!"))
    };

    Levels::load(&mut player)?;

    Ok(())
}

pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
    let mut drawing = rl.begin_drawing(thread);

    // if we don't do this we get epileptic flashing which isn't cute at all
    drawing.clear_background(Color::WHITE);

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