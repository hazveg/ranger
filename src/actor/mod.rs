use bevy::prelude::*;

pub mod player;
pub mod bullet;

#[derive(Component)]
struct Health(f32);

fn move_actors(
    mut actor_query: Query<(&crate::common::Path, &mut Transform)>,
    res_time: Res<Time>,
) {
    for (path, mut transform) in actor_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                player::PlayerPlugin,
                bullet::BulletPlugin,
            ))
            .add_systems(Update, move_actors);
    }
}
