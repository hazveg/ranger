use bevy::prelude::*;

#[derive(Component)]
pub struct Path {
    pub movement: Vec3,
}

impl Path {
    pub fn new() -> Self {
        Path { movement: Vec3::ZERO }
    }
}

#[derive(Resource)]
pub struct DebugTimer(pub Timer);
