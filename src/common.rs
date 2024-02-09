use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Path {
    pub movement: Vec3,
    pub velocity: f32,
}

impl Path {
    pub fn new(velocity: f32) -> Self {
        Path {
            movement: Vec3::ZERO,
            velocity,
        }
    }

    pub fn steering(&mut self, origin: &Vec3, destination: &Vec3) {
        let desired_velocity = (*destination - *origin).normalize_or_zero() * self.velocity;

        self.movement = desired_velocity - origin.normalize_or_zero();
    }
    
    // HOPEFULLY TEMPORARY SOLUTION FOR BULLETS
    pub fn r#static(origin: &Vec3, destination: &Vec3, velocity: f32) -> Self {
        let desired_velocity = (*destination - *origin).normalize_or_zero() * velocity;

        Self {
            movement: desired_velocity - origin.normalize_or_zero(),
            velocity: 1.0,
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
