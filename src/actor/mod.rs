use bevy::{prelude::*, window::PrimaryWindow};
use crate::physics::aabb::*;

pub mod player;
pub mod basic_enemy;
pub mod bullet;

#[derive(Component)]
struct Health(f32);

pub fn move_actors(
    mut actor_query: Query<(
        &crate::common::Path,
        &mut Transform,
        &AABB
    )>,
    res_time: Res<Time>,
) {
    for (path, mut transform, _) in actor_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
    }
}

fn detect_actor_collisions(
    actor_query: Query<(&crate::common::Path, &AABB)>,
    res_time: Res<Time>,
) {
    let actors: Vec<(&crate::common::Path, &AABB)> = actor_query.iter().collect();
    for i in 0..actors.len() {
        if actors[i].0.movement == Vec3::ZERO {
            continue;
        }

        for j in i+1..actors.len() {
            let (path, bounding_box) = actors[i];
            let (_, static_bounding_box) = actors[j];

            let delta = bounding_box.delta(path.movement * res_time.delta_seconds());
            if !static_bounding_box.box_collision(&delta) {
                continue;
            }

            println!("collision");
        }
    }

}

/// The original RANGER didn't have any nifty camera scrolling. So the same has to apply here.
/// As a result, we can't have any actors going out of bounds.
///
/// We will allow a tolerance of the actors width/height, before we start pushing them back.
///
/// The width, height, and position are provided by the bounding box,
/// The borders by the window.
fn confine_actors_to_screen(
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
                detect_actor_collisions.before(move_actors),
                move_actors,
                confine_actors_to_screen,
            ));
    }
}
