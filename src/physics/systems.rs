use bevy::prelude::*;
use crate::actor::bullet::Bullet;
use crate::physics::aabb::AABB;


pub fn update_bounding_box_points(
    mut bounding_box_query: Query<(&mut AABB, &Transform)>,
) {
    for (mut aabb, transform) in bounding_box_query.iter_mut() {
        aabb.point = transform.translation;
    }
}

pub fn debug_bounding_boxes(
    bounding_box_query: Query<&AABB>,
    mut gizmos: Gizmos,
) {
    for bounding_box in bounding_box_query.iter() {
        bounding_box.outline(&mut gizmos)
    }
}

pub fn detect_bullet_collisions(
    bullet_query: Query<Entity, With<Bullet>>,
    // not really meant to affect the player at the moment, i'll figure it out later.
    bounding_box_query: Query<(Entity, &AABB), Without<crate::actor::player::Player>>,
    mut bullet_collision_event: EventWriter<super::BulletCollisionEvent>,
) {
    let bullets: Vec<Entity> = bullet_query.iter().collect();
    let bullet_bounding_boxes: Vec<(Entity, &AABB)> = bounding_box_query.iter()
        .filter(|(entity, _)| bullets.contains(&entity))
        .collect();
    let other_bounding_boxes: Vec<(Entity, &AABB)> = bounding_box_query.iter()
        .filter(|(entity, _)| !bullets.contains(&entity))
        .collect();

    for bullet in &bullet_bounding_boxes {
        for bounding_box in &other_bounding_boxes {
            if !bullet.1.box_collision(bounding_box.1) {
                continue;
            }

            bullet_collision_event.send(super::BulletCollisionEvent(bounding_box.0, bullet.0));
        }
    }
}
