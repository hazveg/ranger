use bevy::prelude::*;

pub mod player;

#[derive(Component)]
struct Health(f32);

fn move_actors(
    mut actor_query: Query<(&crate::common::Path, &mut Transform)>,
) {
    for (path, mut transform) in actor_query.iter_mut() {
        transform.translation += path.movement;
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(player::PlayerPlugin)
            .add_systems(Update, move_actors);
    }
}
