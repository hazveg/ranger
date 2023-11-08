use bevy::prelude::*;

pub mod common;
pub mod actor;
pub mod interface;

fn init(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        ..default()
    });
}

fn move_actors(
    mut actor_query: Query<(&crate::common::Path, &mut Transform)>,
) {
    for (path, mut transform) in actor_query.iter_mut() {
        transform.translation += path.movement;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(actor::ActorPlugin)
        .insert_resource(common::DebugTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .insert_resource(interface::CursorCoordinates(Vec3::ZERO))
        .add_systems(Startup, init)
        .add_systems(Update, (move_actors, interface::update_cursor_position))
        .run();
}
