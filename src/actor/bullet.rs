use bevy::prelude::*;
use ranger_physics::{AABB, Path};

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDropoff(f32);

#[derive(Resource, Default)]
// The cooldown needs to be dynamic, so no Timer
struct ShootCooldown(f32);

#[derive(Component)]
pub struct Hit;

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
        Path::r#static(
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
    bullet_query: Query<(Entity, &Path, &mut Transform), With<Bullet>>,
    actor_query: Query<(Entity, &AABB), Without<super::player::Player>>,
    mut commands: Commands,
    res_time: Res<Time>,
) {
    for (b_entity, path, transform) in bullet_query.iter() {
        let movement_vector = transform.translation + path.movement * res_time.delta_seconds();

        for (a_entity, aabb) in actor_query.iter() {
            if !aabb.intersect_line(transform.translation, movement_vector) {
                continue;
            }

            commands.entity(a_entity).insert(Hit);
            commands.entity(b_entity).insert(Hit);
        }
    }
}

fn slow_down_bullets_that_hit(
    mut bullet_query: Query<(Entity, &mut Path, &Hit), With<Bullet>>,
    mut commands: Commands,
) {
    for (entity, mut path, _) in bullet_query.iter_mut() {
        path.velocity -= 200.0;

        commands.entity(entity).remove::<Hit>();
    }
}

// hopefully only a temporary measure
fn move_bullets(
    mut bullet_query: Query<(&Path, &mut Transform), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (path, mut transform) in bullet_query.iter_mut() {
        transform.translation += path.movement * res_time.delta_seconds();
    }
}

fn lower_bullet_velocity(
    mut bullet_query: Query<(&mut Path, &mut BulletDropoff), With<Bullet>>,
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
    bullet_query: Query<(&Path, Entity), With<Bullet>>,
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
                check_for_collisions.before(move_bullets),
                slow_down_bullets_that_hit,
                move_bullets,
                lower_bullet_velocity,
                // prepare for panics if you don't do this
                remove_stopped_bullets.after(check_for_collisions),
            ));
    }
}
