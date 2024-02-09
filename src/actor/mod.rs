use bevy::{prelude::*, window::PrimaryWindow};
use ranger_physics::AABB;
use crate::physics::Resolve;

pub mod player;
pub mod basic_enemy;
pub mod bullet;

#[derive(Component)]
struct Health(f32);

pub fn move_actors(
    mut actor_query: Query<(
        &crate::common::Path,
        &mut Transform,
        &mut AABB,
    )>,
    res_time: Res<Time>,
) {
    for (path, mut transform, mut aabb) in actor_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
        aabb.point = transform.translation;
    }
}

pub fn detect_collisions(
    actor_query: Query<(Entity, &crate::common::Path, &AABB)>,
    res_time: Res<Time>,
    mut commands: Commands,
) {
    let actors: Vec<(Entity, &crate::common::Path, &AABB)> = actor_query.iter().collect();
    for i in 0..actors.len() {
        if actors[i].1.movement == Vec3::ZERO {
            continue;
        }

        let (entity, path, bounding_box) = actors[i];

        'revolt: for j in i+1..actors.len() {
            let (_, _, static_bounding_box) = actors[j];
            let mut correction = Vec3::ZERO;
            let mut truncated_movement = path.movement.clone();
            
            for k in 1..=100 {
                let path_trunc = path.movement * (k as f32 * 0.01);
                let delta = bounding_box.delta(path_trunc * res_time.delta_seconds());

                if delta.box_collision(&static_bounding_box) {
                    truncated_movement = path_trunc;
                    correction = delta.correct(static_bounding_box);
                    break;
                }

                if k == 10 {
                    break 'revolt;
                }
            }

            commands.entity(entity).insert(Resolve { correction, truncated_movement });
        }
    }
}

fn resolve_collisions(
    mut actor_query: Query<(Entity, &mut crate::common::Path, &Resolve)>,
    res_time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut path, resolve) in actor_query.iter_mut() {
        path.movement = resolve.truncated_movement;
        path.movement = resolve.correction;
        path.movement /= res_time.delta_seconds();
        commands.entity(entity).remove::<Resolve>();
    }
}

/// The original RANGER didn't have any nifty camera scrolling. So the same has to apply here.
/// As a result, we can't have any actors going out of bounds.
///
/// We will allow a tolerance of the actors width/height, before we start pushing them back.
///
/// The width, height, and position are provided by the bounding box,
/// The borders by the window.
fn confine_to_screen(
    mut actor_query: Query<(&mut Transform, &AABB)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if actor_query.is_empty() {
        return;
    }

    let window = window_query.single();
    let base_boundaries = [window.width() / 2.0, window.height() / 2.0];
    
    for (mut transform, aabb) in actor_query.iter_mut() {
        if aabb.point.x > base_boundaries[0] + aabb.width / 2.0 {
            transform.translation.x = base_boundaries[0] + aabb.width / 2.0;
        }

        if aabb.point.x < -(base_boundaries[0] + aabb.width / 2.0) {
            transform.translation.x = -(base_boundaries[0] + aabb.width / 2.0);
        }

        if aabb.point.y > base_boundaries[1] + aabb.height / 2.0 {
            transform.translation.y = base_boundaries[1] + aabb.height / 2.0;
        }

        if aabb.point.y < -(base_boundaries[1] + aabb.height / 2.0) {
            transform.translation.y = -(base_boundaries[1] + aabb.height / 2.0);
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<bullet::HitEvent>()
            .add_plugins((
                player::PlayerPlugin,
                bullet::BulletPlugin,
                basic_enemy::EnemyPlugin,
            ))
            .add_systems(Update, (
                detect_collisions.before(resolve_collisions),
                resolve_collisions.before(move_actors),
                move_actors,
                confine_to_screen,
            ));
    }
}
