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
            .add_systems(Update, move_player);
    }
}

fn init(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: asset_server.load("sprites/sussy.png"),
            ..default()
        },
        Player,
        Path::new(),
        Health(100.0),
    ));
}

fn move_player(
    mut player_query: Query<&mut Path, With<Player>>,
    res_keyboard_input: Res<Input<KeyCode>>,
    res_time: Res<Time>,
) {
    if let Err(_) = player_query.get_single() {
        return;
    }

    let mut movement = Vec3::ZERO;

    if res_keyboard_input.pressed(KeyCode::W) { movement.y += 200.0 }
    if res_keyboard_input.pressed(KeyCode::A) { movement.x -= 200.0 }
    if res_keyboard_input.pressed(KeyCode::S) { movement.y -= 200.0 }
    if res_keyboard_input.pressed(KeyCode::D) { movement.x += 200.0 }

    let mut player_path = player_query.single_mut();
    player_path.adjustment = movement * res_time.delta_seconds();
}

#[derive(Component)]
struct Path {
    adjustment: Vec3,
}

impl Path {
    pub fn new() -> Self {
        Path { adjustment: Vec3::ZERO }
    }
}

fn move_creatures(
    mut creature_query: Query<(&Path, &mut Transform)>,
) {
    for (path, mut transform) in creature_query.iter_mut() {
        println!("{}", path.adjustment);
        transform.translation += path.adjustment;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(DebugTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .add_systems(Startup, init)
        .add_systems(Update, move_creatures)
        .run();
}
