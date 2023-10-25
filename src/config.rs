use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Resource)]
pub struct Config {
    pub title: String,
    pub window: WindowConfig,
    pub bird: BirdConfig,
    pub pipes: PipesConfig,
    pub borders: BordersConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BirdConfig {
    pub speed: f32,
    pub jump_power: f32,
    pub pull_down_power: f32,
    pub collision_radius: f32,
    pub texture_radius: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipesConfig {
    pub interval: f32,
    pub width: f32,
    pub doorway: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BordersConfig {
    pub height: f32,
}

pub fn config() -> &'static Config {
    static INSTANCE: std::sync::OnceLock<Config> = std::sync::OnceLock::new();

    let contents = std::fs::read_to_string("Game.toml").expect("Unable to read config file.");
    let config = toml::from_str::<Config>(&contents).expect("Unable to parse config file.");

    INSTANCE.get_or_init(|| config)
}
