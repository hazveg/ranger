use bevy::{prelude::*, window::PrimaryWindow};
use crate::physics::aabb::*;

pub mod player;
pub mod basic_enemy;
pub mod bullet;

#[derive(Component)]
struct Health(f32);

pub fn move_actors_and_detect_collisions(
    mut actor_query: Query<(
        &crate::common::Path,
        &mut Transform,
        &AABB
    )>,
    res_time: Res<Time>,
) {
    let actors: Vec<(&crate::common::Path, &Transform, &AABB)> = actor_query.iter().collect();
    for i in 0..actors.len() {
        if actors[i].0.movement == Vec3::ZERO {
            continue;
        }

        for j in i+1..actors.len() {
            let (path, _, bounding_box) = actors[i];
            let (_, _, static_bounding_box) = actors[j];

            if !static_bounding_box.box_collision(&bounding_box.delta(path.movement * res_time.delta_seconds())) {
                continue;
            }

            println!("actor collision");
        }
    }

    for (path, mut transform, _) in actor_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
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
                move_actors_and_detect_collisions,
                confine_actors_to_screen,
            ));
    }
}
