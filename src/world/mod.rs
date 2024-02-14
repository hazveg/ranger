use bevy::prelude::*;

mod physics;
mod map;

fn init_grid(
    mut commands: Commands,
) {
    commands.spawn(map::Grid::new(20, 20));
}

fn debug_grid(
    grid_query: Query<&map::Grid>,
    mut gizmos: Gizmos,
) {
    if let Ok(grid) = grid_query.get_single() {
        grid.field_debug(&mut gizmos);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(physics::PhysicsPlugin)
            .add_systems(Startup, init_grid)
            .add_systems(Update, debug_grid);
    }
}
