use bevy::prelude::*;
use ranger_physics::*;

mod physics;
mod map;

fn init_grid(
    mut commands: Commands,
) {
    commands.spawn(map::Grid::new(9, 9));
}

fn debug_grid(
    grid_query: Query<&map::Grid>,
    mut gizmos: Gizmos,
) {
    if let Ok(grid) = grid_query.get_single() {
        grid.field_debug(&mut gizmos);
    }
}

// TODO: actually add the component to the entity
pub fn set_field_coords(
    actor_query: Query<(Entity, &Transform, Option<&AABB>), With<Path>>,
    grid_query: Query<&map::Grid>,
    mut commands: Commands,
) {
    if let Err(_) = grid_query.get_single() {
        return;
    }

    let grid = grid_query.single();

    for (entity, transform, aabb) in actor_query.iter() {
        match aabb {
            Some(aabb) => commands.entity(entity).insert(self::map::FieldCoordinates(grid.associate_aabb(aabb))),
            None => commands.entity(entity).insert(self::map::FieldCoordinates(grid.associate_point(&transform.translation))),
        };
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if !crate::DEBUG {
            app
                .add_systems(Update, (
                    debug_grid,
                ));
        }
        app
            .add_plugins(physics::PhysicsPlugin)
            .add_systems(Startup, init_grid)
            .add_systems(Update, set_field_coords);
    }
}
