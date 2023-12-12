use bevy::prelude::*;

pub mod aabb;


const DEBUG: bool = true;


fn update_bounding_box_points(
    mut bounding_box_query: Query<(&mut self::aabb::AABB, &Transform)>,
) {
    for (mut aabb, transform) in bounding_box_query.iter_mut() {
        aabb.point = transform.translation;
    }
}

fn debug_bounding_boxes(
    bounding_box_query: Query<&aabb::AABB>,
    mut gizmos: Gizmos,
) {
    for bounding_box in bounding_box_query.iter() {
        bounding_box.outline(&mut gizmos)
    }
}


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_bounding_box_points);

        if DEBUG {
            app
                .add_systems(Update, debug_bounding_boxes);
        }
    }
}
