use anyhow::{Result, anyhow};
use raylib::{RaylibHandle, RaylibThread, prelude::{Color, RaylibDraw}};

mod player;

pub use player::instance::PLAYER;

pub fn update(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
    let mut drawing = rl.begin_drawing(thread);

    // if we don't do this we get epileptic flashing which isn't cute at all
    drawing.clear_background(Color::WHITE);

    // this is to ensure "thread safety" (we're running the game on one thread :steamhappy:)
    let mut player = match PLAYER.lock() {
        Ok(p) => p,
        Err(_) => return Err(anyhow!("Error locking PLAYER mutex!")) // please format your errors like this thank you
    };

    player.update(&mut drawing);

    Ok(())
}