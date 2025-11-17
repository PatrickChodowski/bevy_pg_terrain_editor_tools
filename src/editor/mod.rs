use bevy::prelude::*;

pub mod brush;
pub mod pointer;
pub mod vertex;

use crate::editor::pointer::TerrainEditorPointerPlugin;
use crate::editor::brush::TerrainEditorBrushesPlugin;
use crate::editor::vertex::TerrainEditorVertexPlugin;

pub struct BevyPGTerrainEditorPlugin;

impl Plugin for BevyPGTerrainEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TerrainEditorPointerPlugin)
        .add_plugins(TerrainEditorBrushesPlugin)
        .add_plugins(TerrainEditorVertexPlugin)
        ;
    }
}


#[derive(Component)]
pub struct PlaneToEdit{
    pub dims: Vec2
}

impl PlaneToEdit {
    pub fn ray_intersection(
        &self, 
        loc: Vec3, 
        scale: Vec3, 
        origin: Vec3A, 
        direction: Vec3A
    ) -> Option<f32> {

        let min_corner = Vec3A::new(loc.x - self.dims.x*0.5*scale.x, loc.y, loc.z - self.dims.y*0.5*scale.y);
        let max_corner = Vec3A::new(loc.x + self.dims.x*0.5*scale.x, loc.y, loc.z + self.dims.y*0.5*scale.y);

        let inv_dir = direction.recip();
        
        let t1 = (min_corner - origin) * inv_dir;
        let t2 = (max_corner - origin) * inv_dir;
        
        let t_min = Vec3A::min(t1, t2);
        let t_max = Vec3A::max(t1, t2);
        
        let t_enter = t_min.max_element();
        let t_exit = t_max.min_element();
        
        let hit: bool = t_enter <= t_exit && t_exit >= 0.0;
        if hit {
            return Some(t_enter.max(0.0));
        } else {
            // error!("Issue for ray_intersection with {:?}, ray: {:?}", self, ray);
            return None;
        }
    }
}


