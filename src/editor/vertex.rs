use bevy::prelude::*;
use bevy::mesh::VertexAttributeValues;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::color::palettes::css::{BLACK, ORANGE_RED};

use crate::editor::brush::BrushStroke;
use crate::editor::PlaneToEdit;

pub struct TerrainEditorVertexPlugin;

impl Plugin for TerrainEditorVertexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(init_plane_to_edit)
        .add_observer(select_vertex)
        .add_observer(deselect_vertex)
        .add_systems(Update, vertex_transform_changed)
        ;
    }
}

#[derive(Component, Copy, Clone)]
pub struct PlaneVertex {
    pub index: usize,
    pub loc: [f32;3],
    pub clr: [f32;4],
    pub radius: f32,
    pub plane_entity: Entity
}
impl PlaneVertex {
    pub fn new(
        index: usize, 
        loc: &[f32;3], 
        clr: &[f32; 4], 
        radius: f32,
        plane_entity: Entity
    ) -> Self{
        PlaneVertex {
            loc: *loc, 
            clr: *clr, 
            index, 
            radius, 
            plane_entity
        }
    }
}

#[derive(Component)]
pub struct SelectedVertex;



fn init_plane_to_edit(
    trigger:          On<Add, PlaneToEdit>,
    mut commands:     Commands,
    query:            Query<&Mesh3d, With<PlaneToEdit>>,
    mut meshes:       ResMut<Assets<Mesh>>,
    mut materials:    ResMut<Assets<StandardMaterial>>
){

    let Ok(mesh3d) = query.get(trigger.entity) else {return;};
    let Some(mesh) = meshes.get(&mesh3d.0) else {return;};

    let v_pos: Vec<[f32; 3]> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    let mut v_clr: Vec<[f32; 4]> = Vec::new();
    if let Some(attr_vcolor) = mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
        if let VertexAttributeValues::Float32x4(vcolors) = attr_vcolor {
            v_clr = vcolors.to_vec();
        }
    } else {
        v_clr = vec![[1.0, 1.0, 1.0, 1.0]; v_pos.len()];
    }
    let radius: f32 = 1.0;
    let mut vertices: Vec<Entity> = Vec::new();
    let vertex_mesh = Mesh3d(meshes.add(Sphere{radius, ..default()}));
    let vertex_mat = MeshMaterial3d(materials.add(Color::BLACK.with_alpha(0.85)));

    for (index, pos) in v_pos.iter().enumerate(){

        let entity = commands.spawn((
            vertex_mesh.clone(),
            vertex_mat.clone(),
            NotShadowCaster,
            NotShadowReceiver,
            Transform::from_translation(pos.clone().into()).with_scale(Vec3::splat(1.0)),
            PlaneVertex::new(index, pos, &v_clr[index], radius, trigger.entity),
        )).id();

        vertices.push(entity);
    }

    commands.entity(trigger.entity).add_children(&vertices);
}


fn select_vertex(
    trigger:          On<Add, SelectedVertex>,
    mut commands:     Commands,
    mut materials:    ResMut<Assets<StandardMaterial>>,
    mut vertices:     Query<&mut Transform, With<SelectedVertex>>,
    stroke:           Single<&BrushStroke>
){
    commands.entity(trigger.entity).insert(MeshMaterial3d(materials.add(Color::from(ORANGE_RED).with_alpha(0.85))));
    let Ok(mut v_transform) = vertices.get_mut(trigger.entity) else {return;}; 
    stroke.apply(&mut v_transform);
}

fn deselect_vertex(
    trigger:          On<Remove, SelectedVertex>,
    mut commands:     Commands,
    mut materials:    ResMut<Assets<StandardMaterial>>,
){
    commands.entity(trigger.entity).insert(MeshMaterial3d(materials.add(Color::from(BLACK).with_alpha(0.85))));
}


fn vertex_transform_changed(
    mut vertices:   Query<(&mut PlaneVertex, &Transform), (With<SelectedVertex>, Changed<Transform>)>,
    plane_mesh3d:     Single<&Mesh3d, With<PlaneToEdit>>,
    mut meshes:     ResMut<Assets<Mesh>>
){
    let Some(plane_mesh) = meshes.get_mut(&plane_mesh3d.0) else {return;};
    let mut v_pos = plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    for (mut plane_vertex, plane_transform) in vertices.iter_mut(){
        plane_vertex.loc = plane_transform.translation.into();
        v_pos[plane_vertex.index] = plane_vertex.loc;
    }
    plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
}
