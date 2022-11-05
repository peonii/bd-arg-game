use anyhow::Result;
use bd_arg_game::update;

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title(":)")
        .build();

    while !rl.window_should_close() {
        update(&mut rl, &thread)?;
    }

    Ok(())
}
