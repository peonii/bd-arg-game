use std::sync::Mutex;

use super::level::Level;
use once_cell::sync::Lazy;


pub static LEVELS: Lazy<Mutex<Vec<Level>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static LEVELS_LOAD: &'static str = include_str!("maps.yml");