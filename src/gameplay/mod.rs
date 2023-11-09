use bevy::prelude::*;

mod shooting;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(shooting::ShootingPlugin);
    }
}
