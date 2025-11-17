use bevy::prelude::*;
use bevy::picking::pointer::PointerId;
use bevy::picking::hover::HoverMap;
use bevy::window::PrimaryWindow;

use crate::editor::PlaneToEdit;

pub struct TerrainEditorPointerPlugin;

impl Plugin for TerrainEditorPointerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PointerData::new())
            .add_systems(Update, hover_terrain)
        ;
    }
}

fn hover_terrain(
    mut input_data:     ResMut<PointerData>,
    hovermap:           Res<HoverMap>,
    primary:            Single<&Window, With<PrimaryWindow>>,
    camera:             Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    nodes:              Query<Entity, With<Node>>,
    planes:             Query<(&PlaneToEdit, &Transform)>,
) {
    input_data.reset();
    let mut any_ui_hit: Option<Entity> = None;
    let hit_data = hovermap.0.get(&PointerId::Mouse).unwrap();

    if hit_data.len() > 0 {
        let hit_entities: Vec<Entity> = hit_data.keys().cloned().collect::<Vec<Entity>>();
        for entity in hit_entities.iter(){
            if let Ok(_node_entity) = nodes.get(*entity){
                any_ui_hit = Some(*entity);
            }
        }
    }

    let (main_camera, camera_transform) = camera.into_inner();
    let Some(cursor_position) = primary.cursor_position() else {return;};
    input_data.cursor_pos = Some(cursor_position);

    if any_ui_hit.is_some(){
        return;
    }

    if let Ok(ray) = main_camera.viewport_to_world(camera_transform, cursor_position) {
        let ray_origin: Vec3A = ray.origin.into();
        let ray_dir: Vec3A = Vec3A::from(*ray.direction);

        for (plane, plane_transform) in planes.iter(){
            if let Some(distance) = plane.ray_intersection(plane_transform.translation, plane_transform.scale, ray_origin, ray_dir){
                if distance > 0.0 {
                    let position: Vec3 = (ray_origin + ray_dir * distance).into();
                    input_data.world_pos = Some(position);
                    return;
                }
            }
        }
    }
}




#[derive(Resource, Debug)]
pub struct PointerData {
    pub cursor_pos:      Option<Vec2>,
    pub world_pos:       Option<Vec3>,
}


impl PointerData {
    fn new() -> PointerData {
        return Self {
            cursor_pos: None,
            world_pos: None
        };
    }
    fn reset(&mut self) {
        *self = PointerData::new();
    }
}