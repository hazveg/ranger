use bevy::prelude::*;
use ranger_physics::{AABB, Path};


#[derive(Component)]
struct Correction(pub Option<Vec3>);

fn detect_actor_collisions(
    actor_query: Query<(Entity, &AABB, &Path), Without<Correction>>,
    mut commands: Commands,
) {
    let actors: Vec<(Entity, &AABB, &Path)> = actor_query.iter().collect();

    for i in 0..actors.len() {
        let (entity0, aabb0, path0) = actors[i];
        
        if path0.movement != Vec3::ZERO {
            continue;
        }

        for j in i+1..actors.len() {
            let (entity1, aabb1, path1) = actors[j];

            if path1.movement != Vec3::ZERO {
                continue;
            }

            let correction = match aabb0.static_static(aabb1) {
                None => continue,
                Some(correction) => correction,
            };

            commands.entity(entity0).insert(Correction(Some(correction)));
            commands.entity(entity1).insert(Correction(None));
        }
    }
}

fn correct_actor_collisions(
    mut actor_query: Query<(Entity, &mut Transform, &Correction)>,
    mut commands: Commands,
) {
    for (entity, mut transform, correction) in actor_query.iter_mut() {
        // i want them to gently push eachother apart, this'll be visible when enemies spawn
        // by applying the correction directly to the transform, we're not fucking with any
        // movement states
        if let Some(correction) = correction.0 {
            transform.translation += correction;
        }

        commands.entity(entity).remove::<Correction>();
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
        if crate::DEBUG {
            app
                .add_systems(Update, (
                    debug.after(crate::actor::move_actors),
                ));
        }

        app
            .add_systems(Update, (
                detect_actor_collisions.before(correct_actor_collisions),
                correct_actor_collisions.before(crate::actor::move_actors),
            ));
    }
}
