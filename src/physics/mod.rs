use bevy::prelude::*;

pub mod aabb;


fn update_bounding_box_points(
    mut bounding_box_query: Query<(&mut self::aabb::AABB, &Transform)>,
) {
    for (mut aabb, transform) in bounding_box_query.iter_mut() {
        aabb.point = transform.translation;
    }
}


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_bounding_box_points);
    }
}
