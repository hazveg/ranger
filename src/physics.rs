use bevy::prelude::*;
use ranger_physics::AABB;

const DEBUG: bool = true;


pub fn debug(
    bounding_box_query: Query<(&AABB, Option<&crate::actor::bullet::Hit>)>,
    mut gizmos: Gizmos,
) {
    for (bounding_box, hit) in bounding_box_query.iter() {
        let color = match hit {
            Some(_) => Color::RED,
            None => Color::GREEN,
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
