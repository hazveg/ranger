use bevy::prelude::*;
use ranger_physics::AABB;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDropoff(f32);

#[derive(Resource, Default)]
// The cooldown needs to be dynamic, so no Timer
struct ShootCooldown(f32);

#[derive(Event, Debug, PartialEq)]
pub struct HitEvent(pub Entity);


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
        BulletDropoff(0.0),
        crate::common::Path::r#static(
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

pub fn check_for_collisions(
    bullet_query: Query<(&crate::common::Path, &mut Transform), With<Bullet>>,
    actor_query: Query<(Entity, &AABB)>,
    mut hit_event: EventWriter<HitEvent>,
    res_time: Res<Time>,
) {
    for (path, transform) in bullet_query.iter() {
        let movement_vector = transform.translation + path.movement * res_time.delta_seconds();

        for (entity, aabb) in actor_query.iter() {
            for i in 1..=10 {
                if !aabb.point_collision(movement_vector * (i as f32 * 0.1)) {
                    continue;
                }
                
                println!("hit");
                hit_event.send(HitEvent(entity));
                break;
            }
        }
    }
}

// hopefully only a temporary measure
fn move_bullets(
    mut bullet_query: Query<(&crate::common::Path, &mut Transform), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (path, mut transform) in bullet_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
    }
}

fn lower_bullet_velocity(
    mut bullet_query: Query<(&mut crate::common::Path, &mut BulletDropoff), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (mut path, mut bullet_dropoff) in bullet_query.iter_mut() {
        path.velocity -= res_time.delta_seconds() * bullet_dropoff.0 * bullet_dropoff.0;

        bullet_dropoff.0 += 0.05;

        // The borrow checker is the bane of my existance
        let velocity = path.velocity;
        path.movement *= velocity;
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
                check_for_collisions.before(super::detect_collisions),
                move_bullets,
                lower_bullet_velocity,
                remove_stopped_bullets,
            ));
    }
}
