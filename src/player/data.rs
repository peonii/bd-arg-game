use raylib::prelude::*;

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
    fn arrow_movement(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.move_self(self.x, self.y + 1);
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.move_self(self.x, self.y - 1);
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.move_self(self.x + 1, self.y);
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.move_self(self.x - 1, self.y);
        }
    }

    /// Main update function for the player
    pub fn update(&mut self, drawing: &mut RaylibDrawHandle) {
        self.arrow_movement(drawing);

        drawing.draw_circle(self.x, self.y, 15., Color::BLACK);
    }
}