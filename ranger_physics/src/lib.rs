use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
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

    pub fn outline(&self, gizmos: &mut Gizmos, color: Color) {
        let self_vertices = self.vertices();

        gizmos.line(self_vertices[0], Vec3::new(self_vertices[0].x, self_vertices[1].y, 0.0), color);
        gizmos.line(Vec3::new(self_vertices[0].x, self_vertices[1].y, 0.0), self_vertices[1], color);
        gizmos.line(self_vertices[1], Vec3::new(self_vertices[1].x, self_vertices[0].y, 0.0), color);
        gizmos.line(Vec3::new(self_vertices[1].x, self_vertices[0].y, 0.0), self_vertices[0], color);
    }

    // Thank the lord I don't have to do any collision resolution... yet
    // Update: I envy you.
    pub fn delta(&self, movement_vector: Vec3) -> AABB {
        AABB {
            point: Vec3 {
                x: self.point.x + movement_vector.x,
                y: self.point.y + movement_vector.y,
                z: self.point.z + movement_vector.z,
            },
            width: self.width,
            height: self.height,
        }
    }

    pub fn correct(&self, other: &AABB) -> Vec3 {
        let self_vertices = self.vertices();
        let other_vertices = other.vertices();

        let mut correction: Vec3 = Vec3::ZERO;
        
        if self_vertices[1].x > other_vertices[0].x && self_vertices[0].x < other_vertices[0].x {
            correction.x = -(self_vertices[1].x - other_vertices[0].x);
        }
        if self_vertices[0].x < other_vertices[1].x && self_vertices[1].x > other_vertices[1].x {
            correction.x = other_vertices[1].x - self_vertices[0].x;
        }

        if self_vertices[1].y < other_vertices[0].y && self_vertices[0].y > other_vertices[0].y {
            correction.y = other_vertices[0].y - self_vertices[1].y;
        }
        if self_vertices[0].y > other_vertices[1].y && self_vertices[1].y < other_vertices[1].y {
            correction.y = -(self_vertices[0].y - other_vertices[1].y);
        }

        if correction.x.abs() > correction.y.abs() {
            correction.x = 0.0
        } else {
            correction.y = 0.0
        }
        
        correction
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
