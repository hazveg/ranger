use bevy::prelude::*;

#[derive(Component)]
pub struct Path {
    pub movement: Vec3,
    pub velocity: f32,
}

impl Path {
    pub fn new() -> Self {
        Path {
            movement: Vec3::ZERO,
            velocity: 400.0,
        }
    }
}

pub fn get_angle(origin: Vec3, destination: Vec3) -> f32 {
    let x;
    let y;

    if origin.x.is_sign_negative() {
        x = origin.x.abs() + destination.x;
    } else {
        x = -(origin.x) + destination.x;
    }

    if origin.y.is_sign_negative() {
        y = origin.y.abs() + destination.y;
    } else {
        y = -(origin.y) + destination.y;
    }

    y.atan2(x)
}

#[derive(Resource)]
pub struct DebugTimer(pub Timer);
