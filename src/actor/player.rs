use bevy::prelude::*;
use crate::physics::aabb::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
// The cooldown needs to be dynamic, so no Timer
struct ShootCooldown(f32);


const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);


fn shoot(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut ev_shot_fired: EventWriter<crate::actor::bullet::ShotFired>,
    res_cursor_position: Res<crate::interface::CursorCoordinates>,
    res_mouse_input: Res<Input<MouseButton>>,
    mut res_shoot_cooldown: ResMut<ShootCooldown>,
    res_time: Res<Time>,
) {
    if res_shoot_cooldown.0 > 0.0 {
        res_shoot_cooldown.0 -= res_time.delta_seconds();
        return;
    }

    if !res_mouse_input.just_pressed(MouseButton::Left) {
        return;
    } 

    if let Err(_) = player_query.get_single() {
        return;
    }

    let (player, player_transform) = player_query.single();
    
    ev_shot_fired.send(crate::actor::bullet::ShotFired {
        sender: player,
        sender_transform: *player_transform,
        target: res_cursor_position.0,
    });

    res_shoot_cooldown.0 = 0.1;
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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Player,
        AABB::new(Vec3::ZERO, PLAYER_SIZE),
        crate::actor::Health(100.0),
        crate::common::Path::new(),
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


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShootCooldown(0.0))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, rotate_player_to_cursor, shoot));
    }
}
