use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
