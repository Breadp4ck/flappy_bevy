use bevy::prelude::*;

mod config;
mod game;
mod utils;

use game::Game;

const BACKGROUND_COLOR: Color = Color::rgb(0.169, 0.173, 0.185);
const OBSTACLES_COLOR: Color = Color::rgb(0.137, 0.137, 0.149);

fn main() {
    let config = config::config();
    Game::run(config.clone());
}
