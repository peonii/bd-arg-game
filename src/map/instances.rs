use std::sync::Mutex;

use super::level::Level;
use once_cell::sync::Lazy;
use rust_embed::EmbeddedFile;

pub static LEVELS: Lazy<Mutex<Vec<Level>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static LEVELS_LOAD: &str = include_str!("maps.yml");
pub static CURRENT_LEVEL: Lazy<Mutex<Option<Box<EmbeddedFile>>>> = Lazy::new(|| Mutex::new(None));
