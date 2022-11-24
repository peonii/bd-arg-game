use std::{sync::Mutex, time::Instant};

use crate::StartupMode;

use super::level::Level;
use once_cell::sync::Lazy;
use rust_embed::EmbeddedFile;

pub static LEVELS: Lazy<Mutex<Vec<Level>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static LEVELS_LOAD_DAY: &str = include_str!("maps_d.yml");
pub static LEVELS_LOAD_NIGHT: &str = include_str!("maps_n.yml");
pub static LEVELS_LOAD_EYE: &str = include_str!("maps_e.yml");
pub static CURRENT_LEVEL: Lazy<Mutex<Option<Box<EmbeddedFile>>>> = Lazy::new(|| Mutex::new(None));

pub static DIM_LEVEL: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.));

pub static STARTUP_MODE: Lazy<Mutex<StartupMode>> = Lazy::new(|| Mutex::new(StartupMode::Day));

pub static TIMER_START: Lazy<Instant> = Lazy::new(|| Instant::now());