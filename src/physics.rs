use bevy::prelude::*;
use ranger_physics::{AABB, Path};

const DEBUG: bool = true;

#[derive(Component)]
struct Correction(pub Vec3);

fn detect_actor_collisions(
    actor_query: Query<(Entity, &AABB, &Path)>,
    mut commands: Commands,
) {
    let actors: Vec<(Entity, &AABB, &Path)> = actor_query.iter().collect();

    for i in 0..actors.len() {
        let (entity, aabb0, path0) = actors[i];

        for j in i+1..actors.len() {
            let (_, aabb1, path1) = actors[j];

            let (state0, state1) = (path0.movement == Vec3::ZERO, path1.movement == Vec3::ZERO);
            match (state0, state1) {
                (true, true) => if let Some(correction) = aabb0.static_static(aabb1) {
                    commands.entity(entity).insert(Correction(correction));
                },
                (false, false) => if None == aabb0.dynamic_dynamic(path0.movement, aabb1, path1.movement) {},
                _ => {}
            }
        }
    }
}

fn correct_actor_collisions(
    mut actor_query: Query<(Entity, &mut Transform, &Correction)>,
    mut commands: Commands,
    res_time: Res<Time>,
) {
    for (entity, mut transform, correction) in actor_query.iter_mut() {
        // i want them to gently push eachother apart, this'll be visible when enemies spawn
        // by applying the correction directly to the transform, we're not fucking with any
        // movement states
        transform.translation += correction.0 * res_time.delta_seconds();

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
        if DEBUG {
            app
                .add_systems(Update, (
                    debug.after(crate::actor::move_actors),
                    detect_actor_collisions.before(correct_actor_collisions),
                    correct_actor_collisions.before(crate::actor::move_actors),
                ));
        }
    }
}
