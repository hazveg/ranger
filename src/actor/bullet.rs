use bevy::prelude::*;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct BulletDropoff(usize);


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
        BulletDropoff(1),
        crate::common::Path::steering(
            &player_query.single().translation,
            &res_cursor_coordinates.0,
            3000.0,
        ),
        SpriteBundle {
            texture: res_asset_server.load("sprites/sussy.png"),
            transform: *player_query.single(),
            ..default()
        },
    ));

    res_shoot_cooldown.0 = 0.1;
}

fn lower_bullet_velocity(
    mut bullet_query: Query<(&mut crate::common::Path, &BulletDropoff), With<Bullet>>,
) {
    for (mut path, bullet_dropoff) in bullet_query.iter_mut() {
        path.velocity -= 0.0025 * bullet_dropoff.0 as f32 * bullet_dropoff.0 as f32;
        
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
        if path.movement.x.abs() > 75.0 || path.movement.y.abs() > 75.0 {
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
            .add_systems(Update, (spawn_bullets, lower_bullet_velocity, remove_stopped_bullets));
    }
}
