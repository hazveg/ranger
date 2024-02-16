use bevy::prelude::*;
use ranger_physics::AABB;

const DEFAULT_FIELD_WIDTH: f32 = 75.0;
const DEFAULT_FIELD_HEIGHT: f32 = 75.0;

#[derive(Component, Debug)]
pub struct Grid {
    fields: Vec<Field>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn field(&self, row: usize, column: usize) -> &Field {
        let index = ((row-1) * self.columns + column) - 1;

        &self.fields[index]
    }

    fn count(&self, row: &mut usize, column: &mut usize) {
        if *column < self.columns {
            *column += 1;
        } else {
            *column = 1;
            *row += 1;
        }
    }
    
    /// Returns a field index for an AABB
    /// Always >= 1, if 0; not in grid
    /// Already returns a collection, as AABBs can be in multiple fields at a time.
    /// Accordingly if you get a vector containing only 0, you can act as with points
    pub fn associate_aabb(&self, bounding_box: &AABB) -> Vec<(usize, usize)> {
        let mut containing_fields = vec![];
        let mut row = 1;
        let mut column = 1;
        for field in self.fields.iter() {
            if bounding_box.static_static(&field.as_aabb()).is_none() {
                self.count(&mut row, &mut column);
                continue;
            }

            containing_fields.push((row, column));
            self.count(&mut row, &mut column);
        }

        if containing_fields.len() == 0 {
            containing_fields.push((0, 0));
        }

        containing_fields
    }

    /// Returns a field index for a point
    /// Always >= 1, if 0; not in grid
    /// *or my algorithm is shit, that's possible as well*
    pub fn associate_point(&self, point: &Vec3) -> Vec<(usize, usize)> {
        let mut row = 1;
        let mut column = 1;
        for field in self.fields.iter() {
            if !field.as_aabb().point_collision(*point) {
                self.count(&mut row, &mut column);
                continue;
            }

            return vec![(row, column)];
        }

        vec![(0, 0)]
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        let mut fields = vec![];
        
        let x_correction = (columns as f32 / 2.0) * DEFAULT_FIELD_WIDTH;
        let y_correction = (rows as f32 / 2.0) * DEFAULT_FIELD_HEIGHT;

        for i in (0..rows).rev() {
            for j in 0..columns {
                let y = i as f32 * DEFAULT_FIELD_WIDTH + (DEFAULT_FIELD_WIDTH / 2.0) - x_correction;
                let x = j as f32 * DEFAULT_FIELD_HEIGHT + (DEFAULT_FIELD_HEIGHT / 2.0) - y_correction;

                fields.push(Field { point: Vec3::new(x, y, 0.0), width: DEFAULT_FIELD_WIDTH, height: DEFAULT_FIELD_HEIGHT });
            }
        }

        Self { fields, rows, columns }
    }

    pub fn field_debug(&self, gizmos: &mut Gizmos) {
        for field in self.fields.iter() {
            gizmos.rect_2d(field.point.truncate(), 0.0, Vec2::new(field.width, field.height), Color::GREEN);
        }
    }
}

#[derive(Debug)]
struct Field {
    point: Vec3,
    width: f32,
    height: f32,
}

impl Field {
    fn as_aabb(&self) -> AABB {
        AABB::new(self.point, Vec2::new(DEFAULT_FIELD_WIDTH, DEFAULT_FIELD_HEIGHT))
    }
}

#[derive(Component)]
pub struct FieldCoordinates(pub Vec<(usize, usize)>);

#[test]
fn name() {
    fn it_works() {
    }
}
