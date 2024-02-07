use bevy::prelude::*;

pub mod aabb;
mod systems;


const DEBUG: bool = true;


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                systems::update_bounding_box_points.before(crate::actor::detect_actor_collisions),
                systems::update_bounding_box_points.after(crate::actor::move_actors),
            ));
        
        if DEBUG {
            app
                .add_systems(Update, (
                    systems::debug_bounding_boxes,
                ));
        }
    }
}
