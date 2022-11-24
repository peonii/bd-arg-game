#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use bd_arg_game::update;

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("p a t i e n c e")
        .build();

    bd_arg_game::init(&mut rl, &thread)?;

    while !rl.window_should_close() {
        update(&mut rl, &thread)?;
    }

    Ok(())
}
