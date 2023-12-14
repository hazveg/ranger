use bevy::prelude::*;
use crate::physics::aabb::AABB;

#[derive(Event)]
pub struct ShotFired {
    pub sender: Entity,
    pub sender_transform: Transform,
    pub target: Vec3,
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct BulletSpecs {
    dropoff: f32,
    sender: Entity,
}

impl BulletSpecs {
    fn new(sender: Entity) -> BulletSpecs {
        Self { dropoff: 0.0, sender }
    }
}


fn spawn_bullets(
    mut commands: Commands,
    res_asset_server: Res<AssetServer>,
    mut ev_shot_fired: EventReader<ShotFired>,
) {
    for shot_event in ev_shot_fired.read() {
        commands.spawn((
            Bullet,
            AABB::new(shot_event.sender_transform.translation, Vec2::splat(10.0)),
            BulletSpecs::new(shot_event.sender),
            crate::common::Path::steering(
                &shot_event.sender_transform.translation,
                &shot_event.target,
                6000.0,
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                texture: res_asset_server.load("sprites/sussy.png"),
                transform: shot_event.sender_transform,
                ..default()
            },
        ));
    }
}

fn lower_bullet_velocity(
    mut bullet_query: Query<(&mut crate::common::Path, &mut BulletSpecs), With<Bullet>>,
    res_time: Res<Time>,
) {
    for (mut path, mut bullet_specs) in bullet_query.iter_mut() {
        path.velocity -= res_time.delta_seconds() * bullet_specs.dropoff * bullet_specs.dropoff;

        bullet_specs.dropoff += 0.05;
        
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
            .add_event::<ShotFired>()
            .add_systems(Update, (spawn_bullets, lower_bullet_velocity, remove_stopped_bullets));
    }
}
