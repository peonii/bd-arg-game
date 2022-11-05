use std::sync::Mutex;
use once_cell::sync::Lazy;
use super::data::Player;

/// Global instance of the Player class.
pub static PLAYER: Lazy<Mutex<Player>> = Lazy::new(|| Mutex::new(Player::default()));