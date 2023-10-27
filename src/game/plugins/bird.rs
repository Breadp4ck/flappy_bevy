use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::Config;
use crate::game::components::{Doorway, Obstacle, Scores};
use crate::game::{components::Bird, GameCollisionGroups, GameState};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .add_systems(
                Update,
                handle_jump.run_if(in_state(GameState::Game).or_else(in_state(GameState::Pause))),
            )
            .add_systems(
                FixedUpdate,
                (falling, obstacle_collision_check).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Greet), start_with_jump)
            .add_systems(OnExit(GameState::Game), freeze)
            .add_systems(OnExit(GameState::End), start_with_jump)
            .add_systems(OnExit(GameState::End), reset);
    }
}

fn spawn_bird(config: Res<Config>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Bird::default())
        .insert(RigidBody::Dynamic)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(GameCollisionGroups::Bird as u32),
            Group::from_bits_truncate(
                GameCollisionGroups::Obstacle as u32 | GameCollisionGroups::Doorway as u32,
            ),
        ))
        .insert(GravityScale(0.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default())
        .insert(Collider::ball(config.bird.collision_radius))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    config.bird.texture_radius * 2.,
                    config.bird.texture_radius * 2.,
                )),
                ..default()
            },
            texture: asset_server.load("bevy_icon.png"),
            ..default()
        })
        .insert(Transform::IDENTITY);
}

fn handle_jump(
    config: Res<Config>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Bird, &mut Velocity)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (_, mut velocity) in &mut query {
            velocity.linvel.y = config.bird.jump_power;
        }
    }
}

fn start_with_jump(config: Res<Config>, mut query: Query<(&Bird, &mut Velocity)>) {
    for (_, mut velocity) in &mut query {
        velocity.linvel.y = config.bird.jump_power;
    }
}

fn falling(config: Res<Config>, time: Res<Time>, mut query: Query<(&Bird, &mut Velocity)>) {
    let delta = time.delta_seconds();
    let direction = Vec2::NEG_Y * config.bird.pull_down_power;

    let velocity_delta = direction * delta;

    for (_, mut velocity) in &mut query {
        velocity.linvel += velocity_delta;
    }
}

fn obstacle_collision_check(
    mut scores: ResMut<Scores>,
    obstacles_query: Query<&Obstacle>,
    doorways_query: Query<&Doorway>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if obstacles_query.contains(*e1) | obstacles_query.contains(*e2) {
                    next_state.set(GameState::End);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if doorways_query.contains(*e1) || doorways_query.contains(*e2) {
                    scores.0 += 1;
                }
            }
        }
    }
}

fn freeze(mut query: Query<(&Bird, &mut Velocity)>) {
    for (_, mut velocity) in &mut query {
        velocity.linvel = Vec2::ZERO;
    }
}

fn reset(mut scores: ResMut<Scores>, mut query: Query<(&Bird, &mut Transform)>) {
    for (_, mut transform) in &mut query {
        *transform = Transform::IDENTITY;
    }

    scores.0 = 0;
}
