use bevy::prelude::*;
use crate::physics::aabb::AABB;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDropoff(f32);


#[derive(Resource, Default)]
// The cooldown needs to be dynamic, so no Timer
struct ShootCooldown(f32);


fn spawn_bullets(
    player_query: Query<&Transform, With<crate::actor::player::Player>>,
    mut commands: Commands,
    res_mouse_input: Res<Input<MouseButton>>,
    res_cursor_coordinates: Res<crate::interface::CursorCoordinates>,
    mut res_shoot_cooldown: ResMut<ShootCooldown>,
    res_asset_server: Res<AssetServer>,
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

    commands.spawn((
        Bullet,
        AABB::new(player_query.single().translation, Vec2::splat(10.0)),
        BulletDropoff(0.0),
        crate::common::Path::steering(
            &player_query.single().translation,
            &res_cursor_coordinates.0,
            6000.0,
        ),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            texture: res_asset_server.load("sprites/sussy.png"),
            transform: *player_query.single(),
            ..default()
        },
    ));

    res_shoot_cooldown.0 = 0.1;
}

fn lower_bullet_velocity(
    mut bullet_query: Query<(Entity, &mut crate::common::Path, &mut BulletDropoff), With<Bullet>>,
    mut bullet_collision: EventReader<crate::physics::BulletCollisionEvent>,
    res_time: Res<Time>,
) {
    // needs to be done, reading normally "discards" unmatching events => skipped over penetrations
    // yes, i'm upset that this works too.
    let collisions: Vec<&crate::physics::BulletCollisionEvent> = bullet_collision.read().collect();

    for (entity, mut path, mut bullet_dropoff) in bullet_query.iter_mut() {
        path.velocity -= res_time.delta_seconds() * bullet_dropoff.0 * bullet_dropoff.0;

        bullet_dropoff.0 += 0.05;

        // The borrow checker is the bane of my existance
        let velocity = path.velocity;
        path.movement *= velocity;

        for collision in collisions.iter() {
            if collision.1 != entity {
                continue;
            }

            path.velocity -= 25.0 * res_time.delta_seconds();
        }
    }
}

fn remove_stopped_bullets(
    bullet_query: Query<(&crate::common::Path, Entity), With<Bullet>>,
    mut commands: Commands,
) {
    for (path, entity) in bullet_query.iter() {
        if path.velocity > 0.0 {
            continue;
        }

        commands.entity(entity).despawn();
    }
}


pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShootCooldown(0.0))
            .add_systems(Update, (
                spawn_bullets,
                lower_bullet_velocity,
                remove_stopped_bullets,
            ));
    }
}
