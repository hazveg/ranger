use bevy::prelude::*;

pub mod aabb;
mod systems;


const DEBUG: bool = true;

#[derive(Event, Debug)]
pub struct BulletCollisionEvent(pub Entity, pub Entity);


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BulletCollisionEvent>()
            .add_systems(Update, (
                systems::update_bounding_box_points,
                systems::detect_bullet_collisions,
            ));
        
        if DEBUG {
            app
                .add_systems(Update, (
                    systems::debug_bounding_boxes,
                ));
        }
    }
}
