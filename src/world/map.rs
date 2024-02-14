use bevy::prelude::*;

const DEFAULT_FIELD_WIDTH: f32 = 75.0;
const DEFAULT_FIELD_HEIGHT: f32 = 75.0;

#[derive(Component, Debug)]
pub struct Grid {
    fields: Vec<Field>,
    rows: usize,
    columns: usize,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut fields = vec![];
        
        let x_correction = (columns as f32 / 2.0) * DEFAULT_FIELD_WIDTH;
        let y_correction = (rows as f32 / 2.0) * DEFAULT_FIELD_HEIGHT;

        for i in (0..rows).rev() {
            for j in 0..columns {
                let x = i as f32 * DEFAULT_FIELD_WIDTH + (DEFAULT_FIELD_WIDTH / 2.0) - y_correction;
                let y = j as f32 * DEFAULT_FIELD_HEIGHT + (DEFAULT_FIELD_HEIGHT / 2.0) - x_correction;

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

#[test]
fn name() {
    fn it_works() {
    }
}
