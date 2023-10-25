use bevy::prelude::*;

#[derive(Resource)]
pub struct Scores(pub usize);

#[derive(Default, Component)]
pub struct Bird;

#[derive(Default, Component)]
pub struct Pipe;

#[derive(Default, Component)]
pub struct Border;
