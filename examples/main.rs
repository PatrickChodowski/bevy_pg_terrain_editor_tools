
use std::f32::consts::FRAC_PI_2;
use bevy::{color::palettes::css::WHITE, prelude::*};

use bevy_pg_terrain_editor::prelude::{PlaneToEdit, BevyPGTerrainEditorPlugin};

// use crate::editor::MTBEditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight{color: Color::from(WHITE), brightness: 100.0, ..default()})
        .add_plugins(BevyPGTerrainEditorPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes:   ResMut<Assets<Mesh>>,
    mut materials:   ResMut<Assets<StandardMaterial>>
){
    let halfs = Vec2::new(10.0, 10.0);
    commands.spawn(
        (
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, halfs))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(0.5)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            PlaneToEdit{dims: halfs*2.0}
        )
    );

    let mut camera_transform = Transform::from_translation(Vec3::new(4.0, 10.0, 4.0));
    camera_transform.look_at(Vec3::splat(0.0), Vec3::Y);

    commands.spawn(
        (
            Camera3d::default(),
            camera_transform
        )
    );


}