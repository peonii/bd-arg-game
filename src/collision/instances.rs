
use std::sync::Mutex;
use once_cell::sync::Lazy;
use super::collbox::CollisionBox;

pub static BOXES: Lazy<Mutex<Vec<CollisionBox>>> = Lazy::new(|| Mutex::new(Vec::new()));