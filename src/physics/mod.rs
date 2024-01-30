use bevy::prelude::*;

pub mod aabb;
mod systems;


const DEBUG: bool = true;


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                systems::update_bounding_box_points,
            ));
        
        if DEBUG {
            app
                .add_systems(Update, (
                    systems::debug_bounding_boxes,
                ));
        }
    }
}
