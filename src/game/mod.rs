use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::Config;
use crate::BACKGROUND_COLOR;

mod components;
mod plugins;

use components::*;
use plugins::prelude::*;

pub struct Game;

impl Game {
    pub fn run(config: Config) {
        App::new()
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: config.title.clone(),
                        resolution: (config.window.width, config.window.height).into(),
                        ..default()
                    }),
                    ..default()
                }),
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                // RapierDebugRenderPlugin::default(),
                UiPlugin,
                BirdPlugin,
                PipesPlugin,
                BordersPlugin,
            ))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(Scores(0))
            .insert_resource(config)
            .add_state::<GameState>()
            .add_systems(
                Update,
                (
                    pause.run_if(in_state(GameState::Game)),
                    resume.run_if(in_state(GameState::Pause)),
                    start.run_if(in_state(GameState::End).or_else(in_state(GameState::Greet))),
                ),
            )
            .run();
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Greet,
    Pause,
    Game,
    End,
}

enum GameCollisionGroups {
    Bird = 0b0001,
    Obstacle = 0b0010,
    Doorway = 0b0100,
}

fn pause(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Pause);
    }
}

fn resume(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Game);
    }
}

fn start(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Game);
    }
}
