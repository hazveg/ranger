use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AABB {
    pub point: Vec3,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    pub fn new(point: Vec3, size: Vec2) -> Self {
        Self {
            point,
            width: size.x,
            height: size.y,
        }
    }

    /// Doesn't ACTUALLY return the vertices, just the top-left and bottom-right ones
    fn vertices(&self) -> [Vec3; 2] {
        [
            Vec3::new(self.point.x - self.width / 2.0, self.point.y + self.height / 2.0, 0.0),
            Vec3::new(self.point.x + self.width / 2.0, self.point.y - self.height / 2.0, 0.0),
        ]
    }

    pub fn box_collision(&self, other: &AABB) -> bool {
        let self_vertices = self.vertices();
        let other_vertices = other.vertices();

        self_vertices[1].x > other_vertices[0].x && self_vertices[0].x < other_vertices[1].x &&
        self_vertices[1].y < other_vertices[0].y && self_vertices[0].y > other_vertices[1].y
    }

    pub fn point_collision(&self, point: Vec3) -> bool {
        let self_vertices = self.vertices();

        self_vertices[1].x > point.x && self_vertices[0].x < point.x &&
        self_vertices[1].y < point.y && self_vertices[0].y > point.y
    }

    pub fn outline(&self, gizmos: &mut Gizmos) {
        let self_vertices = self.vertices();

        gizmos.line(self_vertices[0], Vec3::new(self_vertices[0].x, self_vertices[1].y, 0.0), Color::GREEN);
        gizmos.line(Vec3::new(self_vertices[0].x, self_vertices[1].y, 0.0), self_vertices[1], Color::GREEN);
        gizmos.line(self_vertices[1], Vec3::new(self_vertices[1].x, self_vertices[0].y, 0.0), Color::GREEN);
        gizmos.line(Vec3::new(self_vertices[1].x, self_vertices[0].y, 0.0), self_vertices[0], Color::GREEN);
    }

    // Thank the lord I don't have to do any collision resolution... yet
}
