use crate::{
    config::Config,
    game::{components::Pipe, GameState},
    utils::{get_random_pipe_v_offset, total_pipes},
    OBSTACLES_COLOR,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pipes)
            .add_systems(
                FixedUpdate,
                teleport_pipes.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnEnter(GameState::Game), run_pipes)
            .add_systems(OnExit(GameState::Game), stop_pipes)
            .add_systems(OnExit(GameState::End), reset);
    }
}

fn spawn_pipes(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let collider = {
        let cuboid = Collider::cuboid(config.pipes.width / 2., config.window.height / 2.);
        let pos_offset = config.window.height / 2. + config.pipes.doorway / 2.;

        Collider::compound(Vec::from([
            (Vec2::Y * pos_offset, 0., cuboid.clone()),
            (Vec2::NEG_Y * pos_offset, 0., cuboid),
        ]))
    };

    let pipe_material = materials.add(ColorMaterial::from(OBSTACLES_COLOR));
    let pipe_mesh: Mesh2dHandle = meshes
        .add(shape::Box::new(config.pipes.width, config.window.height, 1.).into())
        .into();

    let pipe_mesh_offset = Vec3::new(
        0.,
        config.window.height / 2. + config.pipes.doorway / 2.,
        0.,
    );

    for pipe_idx in 0..*total_pipes() {
        commands
            .spawn(Pipe::default())
            .insert(SpatialBundle::default())
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Velocity::default())
            .insert(collider.clone())
            .insert(Transform::from_xyz(
                pipe_idx as f32 * config.pipes.interval
                    + config.pipes.width
                    + config.window.width / 2.,
                get_random_pipe_v_offset(),
                0.,
            ))
            .insert(GlobalTransform::IDENTITY)
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: pipe_mesh.clone(),
                    material: pipe_material.clone(),
                    transform: Transform::from_translation(pipe_mesh_offset),
                    ..default()
                });
                parent.spawn(MaterialMesh2dBundle {
                    mesh: pipe_mesh.clone(),
                    material: pipe_material.clone(),
                    transform: Transform::from_translation(-pipe_mesh_offset),
                    ..default()
                });
            });
    }
}

fn stop_pipes(mut query: Query<(&Pipe, &mut Velocity)>) {
    for (_, mut velocity) in &mut query {
        velocity.linvel = Vec2::ZERO;
    }
}

fn run_pipes(config: Res<Config>, mut query: Query<(&Pipe, &mut Velocity)>) {
    for (_, mut velocity) in &mut query {
        velocity.linvel = Vec2::NEG_X * config.bird.speed;
    }
}

fn teleport_pipes(config: Res<Config>, mut query: Query<(&Pipe, &mut Transform)>) {
    let left_unvisible_offset = -config.window.width / 2. - config.pipes.width / 2.;

    for (_, mut transform) in &mut query {
        if transform.translation.x < left_unvisible_offset {
            transform.translation.x += config.pipes.interval * *total_pipes() as f32;
            transform.translation.y = get_random_pipe_v_offset();
        }
    }
}

fn reset(config: Res<Config>, mut query: Query<(&Pipe, &mut Transform)>) {
    for (pipe_idx, (_, mut transform)) in query.iter_mut().enumerate() {
        transform.translation = Vec3::new(
            pipe_idx as f32 * config.pipes.interval + config.pipes.width + config.window.width / 2.,
            get_random_pipe_v_offset(),
            0.,
        );
    }
}
