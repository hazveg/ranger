use bevy::prelude::*;
use ranger_physics::AABB;

#[derive(Component)]
pub struct Player;


const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Player,
        AABB::new(Vec3::ZERO, PLAYER_SIZE),
        crate::actor::Health(100.0),
        crate::common::Path::new(200.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: asset_server.load("sprites/sussy.png"),
            ..default()
        },
    ));
}

fn move_player(
    mut player_query: Query<&mut crate::common::Path, With<Player>>,
    res_keyboard_input: Res<Input<KeyCode>>,
) {
    if let Err(_) = player_query.get_single() {
        return;
    }

    let mut movement = Vec3::ZERO;
    let mut player_path = player_query.single_mut();

    if res_keyboard_input.pressed(KeyCode::W) { movement.y += player_path.velocity }
    if res_keyboard_input.pressed(KeyCode::A) { movement.x -= player_path.velocity }
    if res_keyboard_input.pressed(KeyCode::S) { movement.y -= player_path.velocity }
    if res_keyboard_input.pressed(KeyCode::D) { movement.x += player_path.velocity }

    player_path.movement = movement;
}

fn rotate_player_to_cursor(
    mut player_query: Query<&mut Transform, With<Player>>,
    res_cursor_position: Res<crate::interface::CursorCoordinates>,
) {
    if let Err(_) = player_query.get_single() {
        return;
    }

    let mut player_transform = player_query.single_mut();
    let angle = crate::common::get_angle(
        player_transform.translation,
        res_cursor_position.0,
    );

    player_transform.rotation = Quat::from_rotation_z(angle);
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player.before(super::detect_collisions), rotate_player_to_cursor));
    }
}
