use bevy::prelude::*;

pub mod player;

#[derive(Component)]
struct Health(f32);

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin);
    }
}
