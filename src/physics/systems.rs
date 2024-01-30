use bevy::prelude::*;
use crate::physics::aabb::AABB;


pub fn update_bounding_box_points(
    mut bounding_box_query: Query<(&mut AABB, &Transform)>,
) {
    for (mut aabb, transform) in bounding_box_query.iter_mut() {
        aabb.point = transform.translation;
    }
}

pub fn debug_bounding_boxes(
    bounding_box_query: Query<&AABB>,
    mut gizmos: Gizmos,
) {
    for bounding_box in bounding_box_query.iter() {
        bounding_box.outline(&mut gizmos)
    }
}
