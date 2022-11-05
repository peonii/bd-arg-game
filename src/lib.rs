use anyhow::{Result, anyhow};
use collision::{collbox::CollisionBox, instances::BOXES};
use raylib::{RaylibHandle, RaylibThread, prelude::{Color, RaylibDraw}};

mod player;
mod collision;

pub use player::instance::PLAYER;

pub fn init(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Result<()> {
    let new_cb = CollisionBox::new(0, 0, 10, 1000);

    CollisionBox::register(new_cb)?;

    //TODO: add automatic collision box initialization from map file
    /////// map file should preferably be a .json file with some nice format so i can ser/deser it easily

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

    {
        let boxes = match BOXES.lock() {
            Ok(b) => b,
            Err(_) => return Err(anyhow!("Error locking BOXES mutex!"))
        };

        for cb in boxes.iter() {
            cb.draw(&mut drawing);
        }
    }

    


    Ok(())
}