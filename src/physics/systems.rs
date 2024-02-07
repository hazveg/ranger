use bevy::prelude::*;
use crate::physics::aabb::AABB;

use crate::actor::bullet::HitEvent;

pub fn debug_bounding_boxes(
    bounding_box_query: Query<(Entity, &AABB)>,
    mut hitevent: EventReader<HitEvent>,
    mut gizmos: Gizmos,
) {
    let hit_events: Vec<Entity> = hitevent.read().map(|ev| ev.0).collect();
    for (entity, bounding_box) in bounding_box_query.iter() {
        let color = match hit_events.contains(&entity) {
            true => Color::RED,
            false => Color::GREEN,
        };

        bounding_box.outline(&mut gizmos, color);
    }
}
