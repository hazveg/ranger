use bevy::prelude::*;
use ranger_physics::AABB;
use crate::actor::bullet::HitEvent;

const DEBUG: bool = true;


pub fn debug(
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


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if DEBUG {
            app
                .add_systems(Update, (
                    debug.after(crate::actor::move_actors),
                ));
        }
    }
}
