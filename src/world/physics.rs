use bevy::prelude::*;
use ranger_physics::{AABB, Path};


#[derive(Component)]
struct Correction(Option<Vec3>);

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

pub fn detect_actor_collisions(
    actor_query: Query<(&AABB, &Path)>,
) {
    let actors: Vec<(&AABB, &Path)> = actor_query.iter().collect();
    for i in 0..actors.len() {
        let (first_aabb, first_path) = actors[i];

        for j in 0..actors.len() {
            if j == i {
                continue;
            }

            let (second_aabb, second_path) = actors[j];

            println!("first: {}, second: {}", first_path.movement, second_path.movement);
            if let true = AABB::is_colliding(first_aabb, first_path, second_aabb, second_path) {
                println!("yippie");
            }
        }
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
        app.add_systems(Update, detect_actor_collisions);
    }
}
