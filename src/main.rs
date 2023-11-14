use bevy::prelude::*;

pub mod common;
pub mod actor;
pub mod interface;
pub mod ai;

fn init(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..default()
                }),
            ..default()
            })
            .build(),
        )
        .add_plugins((
            actor::ActorPlugin,
        ))
        .insert_resource(common::DebugTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .insert_resource(interface::CursorCoordinates(Vec3::ZERO))
        .add_systems(Startup, init)
        .add_systems(Update, interface::update_cursor_position)
        .run();
}
