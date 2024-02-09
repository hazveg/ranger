use bevy::prelude::*;
use ranger_physics::AABB;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Component)]
pub struct BasicEnemy;


const BASIC_ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn spawn_enemy(
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
        crate::common::Path::new(),
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


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_enemy);
    }
}
