use bevy::{prelude::*, sprite::*};
use bevy_rapier2d::prelude::*;

use crate::{config::Config, game::components::Border, OBSTACLES_COLOR};

pub struct BordersPlugin;

impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_borders);
    }
}
/// Create border obstacles at the top and the bottom of the window.
fn spawn_borders(
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let collider = {
        let cuboid = Collider::cuboid(config.window.width / 2., config.borders.height / 2.);
        let pos_offset = config.window.height / 2. - config.borders.height / 2.;

        Collider::compound(Vec::from([
            (Vec2::Y * pos_offset, 0., cuboid.clone()),
            (Vec2::NEG_Y * pos_offset, 0., cuboid),
        ]))
    };

    let border_material = materials.add(ColorMaterial::from(OBSTACLES_COLOR));
    let border_mesh: Mesh2dHandle = meshes
        .add(shape::Box::new(config.window.width, config.borders.height, 2.).into())
        .into();

    let border_mesh_offset = Vec3::new(
        0.,
        config.window.height / 2. - config.borders.height / 2.,
        0.,
    );

    commands
        .spawn(Border::default())
        .insert(SpatialBundle::default())
        .insert(RigidBody::Fixed)
        .insert(collider.clone())
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: border_mesh.clone(),
                material: border_material.clone(),
                transform: Transform::from_translation(border_mesh_offset),
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {
                mesh: border_mesh,
                material: border_material,
                transform: Transform::from_translation(-border_mesh_offset),
                ..default()
            });
        });
}
