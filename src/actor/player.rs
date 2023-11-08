use bevy::prelude::*;

#[derive(Component)]
struct Player;

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
        crate::common::Path::new(),
        crate::actor::Health(100.0),
    ));
}

fn move_player(
    mut player_query: Query<&mut crate::common::Path, With<Player>>,
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
    player_path.movement = movement * res_time.delta_seconds();
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
