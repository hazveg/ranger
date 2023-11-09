use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource, Default)]
// The cooldown needs to be dynamic, so no Timer
struct ShootCooldown(f32);

fn spawn_bullets(
    player_query: Query<&Transform, With<crate::actor::player::Player>>,
    res_mouse_input: Res<Input<MouseButton>>,
    mut res_shoot_cooldown: ResMut<ShootCooldown>,
    res_time: Res<Time>,
) {
    if res_shoot_cooldown.0 > 0.0 {
        res_shoot_cooldown.0 -= res_time.delta_seconds();
        return;
    }

    if res_mouse_input.just_pressed(MouseButton::Left) {
        println!("shooting");
        res_shoot_cooldown.0 = 0.5;
    }
}


pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShootCooldown(0.0))
            .add_systems(Update, spawn_bullets);
    }
}
