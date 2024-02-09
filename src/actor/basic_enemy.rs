use bevy::prelude::*;
use ranger_physics::AABB;
use ranger_ai::Target;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Component)]
pub struct BasicEnemy;


const BASIC_ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res_enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    res_time: Res<Time>,
) {
    if !res_enemy_spawn_timer.0.tick(res_time.delta()).just_finished() {
        return;
    }

    commands.spawn((
        BasicEnemy,
        AABB::new(Vec3::ZERO, BASIC_ENEMY_SIZE),
        crate::actor::Health(50.0),
        crate::common::Path::new(150.0),
        Target::new(None),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(BASIC_ENEMY_SIZE),
                ..default()
            },
            texture: asset_server.load("sprites/enemy_placeholder.png"),
            ..default()
        },
    ));
}

fn detect_player(
    player_query: Query<&Transform, With<super::player::Player>>,
    mut enemy_query: Query<(&mut Target, &Transform), With<BasicEnemy>>,
) {
    if player_query.get_single().is_err() {
        return;
    }

    let player_transform = player_query.single();

    for (mut enemies_target, enemy_transform) in enemy_query.iter_mut() {
        if player_transform.translation.distance(enemy_transform.translation) > 300.0 {
            enemies_target.remove_target();
            continue;
        }

        enemies_target.set_target(player_transform.translation);
    }
}

fn focus_on_target(
    mut enemy_query: Query<(&Target, &mut Transform), With<BasicEnemy>>,
) {
    for (enemies_target, mut enemy_transform) in enemy_query.iter_mut() {
        if !enemies_target.has_target() {
            continue;
        }

        let angle = crate::common::get_angle(
            enemy_transform.translation,
            enemies_target.point.unwrap()
        );

        enemy_transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn pursue_target(
    mut enemy_query: Query<(&Target, &Transform, &mut crate::common::Path), With<BasicEnemy>>,
) {
    for (enemies_target, transform, mut path) in enemy_query.iter_mut() {
        if !enemies_target.has_target() {
            continue;
        }

        path.steering(
            &transform.translation,
            &enemies_target.point.unwrap()
        );
    }
}

fn hit_by_bullet(
    mut enemy_query: Query<(Entity, &mut super::Health), With<BasicEnemy>>,
    mut hitevent: EventReader<super::bullet::HitEvent>,
) {
    let hit_events: Vec<Entity> = hitevent.read().map(|ev| ev.0).collect();

    for (entity, mut health) in enemy_query.iter_mut() {
        if !hit_events.contains(&entity) {
            continue;
        }

        health.0 = 0.0;
    }
}

fn despawn(
    enemy_query: Query<(Entity, &super::Health), With<BasicEnemy>>,
    mut commands: Commands,
) {
    for (entity, health) in enemy_query.iter() {
        if health.0 > 0.0 {
            continue;
        }

        commands.entity(entity).despawn();
    }
}


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Update, (
                spawn,
                detect_player,
                focus_on_target,
                pursue_target,
                hit_by_bullet.after(super::bullet::check_for_collisions),
                despawn,
            ));
    }
}
