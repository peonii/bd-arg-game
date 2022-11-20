use anyhow::{Result, anyhow};
use raylib::prelude::*;

use crate::{collision::instances::BOXES, map::level::Level};

pub struct Player {
    x: i32,
    y: i32
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 100, y: 100
        }
    }
}

impl Player {
    /// Move the player to the specified coordinates.
    fn move_self(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Handle player arrow movement
    /// 
    /// This function handles movement with arrow keys
    /// 
    /// NOTE: We are not going to add support for WASD because why would we
    fn arrow_movement(&mut self, rl: &RaylibHandle) -> Result<()> {
        let mut new_x = self.x;
        let mut new_y = self.y;

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            new_y += 1;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            new_y -= 1;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            new_x += 1;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            new_x -= 1;
        }

        let mut new_area = String::new();

        {
            let boxes = match BOXES.lock() {
                Ok(b) => b,
                Err(_) => return Err(anyhow!("Error locking BOXES mutex"))
            };


            for cb in boxes.iter() {
                if cb.collides_with(new_x, new_y, 15, 15) {
                    if cb.portal_to.is_empty() {
                        return Ok(())
                    } else {
                        new_area = cb.portal_to.clone();
                    }
                }
            }
        }


        if !new_area.is_empty() {
            Level::load(&new_area, self)?;
        } else {
            self.move_self(new_x, new_y);
        }

        Ok(())
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Main update function for the player
    pub fn update(&mut self, drawing: &mut RaylibDrawHandle) -> Result<()> {
        self.arrow_movement(drawing)?;

        drawing.draw_circle(self.x, self.y, 15., Color::BLACK);

        Ok(())
    }
}