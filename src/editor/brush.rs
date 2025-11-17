use bevy::prelude::*;
use bevy::color::palettes::tailwind::BLUE_500;
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;
use bevy::picking::pointer::PointerId;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::platform::collections::HashMap;

use crate::editor::vertex::{PlaneVertex, SelectedVertex};
use crate::editor::pointer::PointerData;


pub struct TerrainEditorBrushesPlugin;

impl Plugin for TerrainEditorBrushesPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(BrushSettings::default())
        .add_observer(brush_drag_start)
        .add_observer(brush_drag)
        .add_observer(brush_drag_end)
        ;
    }
}


fn brush_drag_start(
    trigger:            On<Pointer<DragStart>>,
    window_entity:      Single<Entity, With<PrimaryWindow>>,
    mut commands:       Commands,
    mut meshes:         ResMut<Assets<Mesh>>,
    mut materials:      ResMut<Assets<StandardMaterial>>,
    brush_settings:     Res<BrushSettings>,
    brushes:            Query<Entity, With<BrushStroke>>,
    pointer:            Res<PointerData>
){

    if trigger.entity != *window_entity {
        return;
    }

    if brushes.iter().len() > 0 {
        for entity in brushes.iter(){
            commands.entity(entity).despawn();
        }
    }

    let Some(world_pos) = pointer.world_pos else {return;};

    if trigger.pointer_id == PointerId::Mouse {
        if trigger.button == PointerButton::Primary {
            commands.spawn((
                Mesh3d(meshes.add(Circle::new(brush_settings.radius))),
                MeshMaterial3d(materials.add(Color::from(BLUE_500).with_alpha(0.4))),
                Transform::from_xyz(world_pos.x, world_pos.y + 1.0, world_pos.z)
                          .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                BrushStroke::new(brush_settings.typ.clone(), brush_settings.radius),
                NotShadowCaster,
                NotShadowReceiver
            ));
        }
    }
}

fn brush_drag(
    trigger:         On<Pointer<Drag>>,
    window_entity:   Single<Entity, With<PrimaryWindow>>,
    mut commands:    Commands,
    strokes:         Single<(&mut Transform, &mut BrushStroke), Without<PlaneVertex>>,
    pointer:         Res<PointerData>,
    vertices:        Query<(Entity, &GlobalTransform, &PlaneVertex), Without<SelectedVertex>>
){
    if trigger.entity != *window_entity {
        return;
    }

    let Some(world_pos) = pointer.world_pos else {return;};
    let (mut stroke_transform, mut stroke) = strokes.into_inner();

    stroke_transform.translation = Vec3::new(world_pos.x, world_pos.y + 1.0, world_pos.z);
    stroke.paint(&world_pos, &vertices, &mut commands);
}



fn brush_drag_end(
    trigger:        On<Pointer<DragEnd>>,
    window_entity:  Single<Entity, With<PrimaryWindow>>,
    mut commands:   Commands,
    strokes:        Single<(Entity, &BrushStroke)>,
    selected_vertices: Query<Entity, With<SelectedVertex>>
    // mut changes:    Option<ResMut<Changes>>
){


    if trigger.entity != *window_entity {
        return;
    }

    let (stroke_entity, stroke) = strokes.into_inner();
    
    if trigger.pointer_id == PointerId::Mouse {
        if trigger.button == PointerButton::Primary {
             commands.entity(stroke_entity).despawn();
            // let mut cts: Vec<ChangeSpawn> = Vec::with_capacity(1000);
            for (_k, v) in stroke.data.iter(){
                match v {
                    // StrokeTest::Positive(ct) => {
                    StrokeTest::Positive => {
                        // cts.push(ct.clone());
                    }
                    _ => {}
                }
            }
            // changes.as_mut().unwrap().record_spawn(cts);

            for v_entity in selected_vertices.iter(){
                commands.entity(v_entity).remove::<SelectedVertex>();
            }

        }
    }
}




#[derive(Component)]
pub struct BrushStroke{
    typ:    BrushType,
    radius: f32,
    data:   HashMap<(u32, u32), StrokeTest>
}
impl BrushStroke {
    fn new(typ: BrushType, radius: f32) -> Self {
        Self {
            typ, 
            radius, 
            data: HashMap::with_capacity(2000)
        }
    }

    fn paint(
        &mut self, 
        world_pos:      &Vec3,
        vertices:       &Query<(Entity, &GlobalTransform, &PlaneVertex), Without<SelectedVertex>>,
        commands:       &mut Commands
    ) {
        for (v_entity, v_transform, vertex) in vertices.iter(){
            let v_loc = v_transform.translation().xz();
            let dist = world_pos.xz().distance(v_loc);
            let limit = self.radius + vertex.radius;
            if dist <= limit{
                commands.entity(v_entity).insert(SelectedVertex);
            }
        }

        match self.typ.clone() {
            BrushType::Heights => {}
        }
    }

    pub(crate) fn apply(&self, transform: &mut Transform){
        transform.translation.y += 0.5;
    }

}

enum StrokeTest {
    Negative,
    // Positive(ChangeSpawn)
    Positive
}

#[derive(Clone)]
pub enum BrushType {
    Heights
}

impl BrushType {
    pub fn to_str(&self) -> &'static str {
        match self {
            BrushType::Heights => {"heights"}
        }
    }
}



#[derive(Resource)]
pub struct BrushSettings {
    pub typ: BrushType,
    pub radius: f32
}

impl Default for BrushSettings {
    fn default() -> Self {
        BrushSettings {
            typ: BrushType::Heights,
            radius: 1.0
        }
    }
}
