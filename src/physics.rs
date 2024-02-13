use bevy::prelude::*;
use ranger_physics::{AABB, Path};

const DEBUG: bool = true;


fn detect_actor_collisions(
    actor_query: Query<(&AABB, &Path)>,
) {
    let actors: Vec<(&AABB, &Path)> = actor_query.iter().collect();

    for i in 0..actors.len() {
        let (aabb0, path0) = actors[i];

        for j in i+1..actors.len() {
            let (aabb1, path1) = actors[j];

            let (state0, state1) = (path0.movement == Vec3::ZERO, path1.movement == Vec3::ZERO);
            match (state0, state1) {
                (true, true) => println!("{:?}", aabb0.static_static(aabb1)),
                _ => {}
            }
        }
    }
}

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
                    detect_actor_collisions,
                ));
        }
    }
}
