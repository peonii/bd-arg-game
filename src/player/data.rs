use anyhow::{anyhow, Result};
use raylib::prelude::*;

use crate::{collision::instances::BOXES, map::{level::Level, image::GameImage}};

enum PlayerDirection {
    UP,
    LEFT,
    RIGHT,
    DOWN
}

pub struct Player {
    x: i32,
    y: i32,
    direction: PlayerDirection,
    current_texture: Option<Texture2D>,
    book: bool,
    book_tex: Option<Texture2D>
}

impl Default for Player {
    fn default() -> Self {
        Self { x: 100, y: 100, direction: PlayerDirection::UP, current_texture: None, book: false, book_tex: None }
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
            new_y += 3;
            self.direction = PlayerDirection::DOWN;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            new_y -= 3;
            self.direction = PlayerDirection::UP;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            new_x += 3;
            self.direction = PlayerDirection::RIGHT;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            new_x -= 3;
            self.direction = PlayerDirection::LEFT;
        }

        let mut new_area = String::new();

        {
            let boxes = match BOXES.lock() {
                Ok(b) => b,
                Err(_) => return Err(anyhow!("Error locking BOXES mutex")),
            };

            let boxes_iter = boxes.iter();
            for cb in boxes_iter {
                if cb.collides_with(new_x, new_y, 15, 15) {
                    if cb.portal_to == "book" {
                        self.book = true;
                    } else {
                        self.book = false;
                    }

                    if cb.portal_to.is_empty() {
                        return Ok(());
                    }
                    new_area = cb.portal_to.clone();
                }
            }
        }

        if new_area.is_empty() {
            self.move_self(new_x, new_y);
        } else {
            Level::load(&new_area, self, rl)?;
        }

        Ok(())
    }

    pub const fn get_x(&self) -> i32 {
        self.x
    }

    pub const fn get_y(&self) -> i32 {
        self.y
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn load_text(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<()> {
        let texture = match self.direction {
            PlayerDirection::UP => GameImage::get("player/player_up_idle.png"),
            PlayerDirection::DOWN => GameImage::get("player/player_down_idle.png"),
            PlayerDirection::RIGHT => GameImage::get("player/player_right_idle.png"),
            PlayerDirection::LEFT => GameImage::get("player/player_left_idle.png"),
        };

        let texture = match texture {
            Some(t) => t,
            None => return Err(anyhow!("fuck"))
        };

        let data_size: i32 = texture.data.len().try_into()?;

        let img = match Image::load_image_from_mem(
            ".png",
            &Vec::from(texture.data.clone()),
            data_size,
        ) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to load image!")),
        };

        let tex = match rl.load_texture_from_image(thread, &img) {
            Ok(t) => t,
            Err(_) => return Err(anyhow!("asdfasdfasdf"))
        };

        self.current_texture = Some(tex);

        let book_t = match GameImage::get("book.png") {
            Some(b) => b,
            None => return Err(anyhow!("fuck"))
        };

        let data_size_b: i32 = book_t.data.len().try_into()?;

        let img_b = match Image::load_image_from_mem(".png", &Vec::from(book_t.data.clone()), data_size_b) {
            Ok(i) => i,
            Err(_) => return Err(anyhow!("Failed to load image!")),
        };

        let tex_b = match rl.load_texture_from_image(thread, &img_b) {
            Ok(t) => t,
            Err(_) => return Err(anyhow!("asdfasdfasdf"))
        };

        self.book_tex = Some(tex_b);

        Ok(())
    }

    /// Main update function for the player
    pub fn update(&mut self, drawing: &mut RaylibDrawHandle) -> Result<()> {
        self.arrow_movement(drawing)?;

        let tex = match &self.current_texture {
            Some(t) => t,
            None => return Err(anyhow!("oops"))
        };

        drawing.draw_texture(tex, self.x - tex.width() / 2, self.y - tex.height() / 2, Color::WHITE);

        if self.book {
            let book_tex = match &self.book_tex {
                Some(t) => t,
                None => return Err(anyhow!("oops"))
            };

            drawing.draw_texture(book_tex, 120, 295, Color::BLACK);
            drawing.draw_texture(book_tex, 100, 275, Color::WHITE);

            self.book = false;
        }

        Ok(())
    }
}
