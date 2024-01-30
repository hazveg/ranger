use bevy::prelude::*;

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

/*fn hitscan(
    bullet_query: Query<&Transform, With<Bullet>>,
    actor_query: Query<(Entity, &crate::physics::aabb::AABB), Without<crate::actor::player::Player>>,
    mut hitevent: EventWriter<super::HitEvent>,
) {
    for bullet_transform in bullet_query.iter() {
        for (entity, aabb) in actor_query.iter() {
            if !aabb.point_collision(bullet_transform.translation) {
                continue;
            }

            hitevent.send(super::HitEvent(entity))
        }
    }
}*/

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
                //hitscan.before(crate::actor::move_actors),
                spawn_bullets,
                lower_bullet_velocity,
                remove_stopped_bullets,
            ));
    }
}
