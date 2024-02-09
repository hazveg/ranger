use bevy::prelude::*;
use ranger_physics::AABB;
use crate::actor::bullet::HitEvent;

const DEBUG: bool = true;

#[derive(Component)]
pub struct Resolve {
    pub correction: Vec3,
    pub truncated_movement: Vec3,
}


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

fn detect_collisions(
    bounding_box_query: Query<(Entity, &AABB)>,
    mut commands: Commands,
) {
    let bounding_boxes: Vec<(Entity, &AABB)> = bounding_box_query.iter().collect();

    for i in 0..bounding_boxes.len() {
        let (entity, aabb0) = bounding_boxes[i];
        
        for j in i+1..bounding_boxes.len() {
            let (_, aabb1) = bounding_boxes[j];

            if !aabb0.box_collision(aabb1) {
                continue;
            }

            let correction = aabb0.correct(aabb1);
            commands.entity(entity).insert(Resolve { correction, truncated_movement: Vec3::ZERO });
        }
    }
}

fn resolve_collisions(
    mut bounding_box_query: Query<(Entity, &Resolve, &mut AABB, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, resolve, mut aabb, mut transform) in bounding_box_query.iter_mut() {
        aabb.point += resolve.correction;
        transform.translation = aabb.point;

        commands.entity(entity).remove::<Resolve>();
    }
}


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if DEBUG {
            app
                .add_systems(Update, (
                    debug.after(crate::actor::move_actors),
                    detect_collisions,
                    resolve_collisions,
                ));
        }
    }
}
