use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(f32);

#[derive(Resource)]
struct DebugTimer(Timer);

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, debug_player);
    }
}

fn hello_world() {
    println!("i want to kill myself");
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Health(100.0),
    ));
}

fn debug_player(
    player_query: Query<&Health, With<Player>>,
    mut res_debug_timer: ResMut<DebugTimer>,
    res_time: Res<Time>,
) {
    if !res_debug_timer.0.tick(res_time.delta()).just_finished() {
        return;
    }

    for health in player_query.iter() {
        println!("Player is at {} HP", health.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(DebugTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .add_systems(Startup, hello_world)
        .run();
}
