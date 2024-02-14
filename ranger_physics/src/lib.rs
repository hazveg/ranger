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

    pub fn steering(&mut self, origin: &Vec3, destination: &Vec3, delta_time: f32) {
        let desired_velocity = (*destination - *origin).normalize_or_zero() * self.velocity;

        self.movement = (desired_velocity - origin.normalize_or_zero()) * delta_time;
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

struct Points {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub d: Vec3,
}

impl Points {
    fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Self {
        Points { a, b, c, d }
    }
}

struct Sides {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32
}

impl Sides {
    fn new(left: f32, bottom: f32, right: f32, top: f32) -> Self{
        Sides { left, bottom, right, top }
    }
}

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

    /// The indices are defined counterclockwise
    ///     3
    ///   +---+
    /// 0 |   | 2
    ///   +---+
    ///     1
    fn sides(&self) -> Sides {
        Sides::new(
            self.point.x - self.width / 2.0, self.point.y - self.height / 2.0,
            self.point.x + self.width / 2.0, self.point.y + self.height / 2.0,
        )
    }

    /// The points are also defined counterclockwise
    ///
    /// A     D
    ///  +---+
    ///  |   |
    ///  +---+
    /// B     C
    fn corners(&self) -> Points {
        Points::new(
            Vec3::new(self.point.x - self.width / 2.0, self.point.y + self.height / 2.0, self.point.z),
            Vec3::new(self.point.x - self.width / 2.0, self.point.y - self.height / 2.0, self.point.z),
            Vec3::new(self.point.x + self.width / 2.0, self.point.y - self.height / 2.0, self.point.z),
            Vec3::new(self.point.x + self.width / 2.0, self.point.y + self.height / 2.0, self.point.z),
        )
    }

    pub fn outline(&self, gizmos: &mut Gizmos, color: Color) {
        let this = self.corners();

        gizmos.line(this.a, this.b, color);
        gizmos.line(this.b, this.c, color);
        gizmos.line(this.c, this.d, color);
        gizmos.line(this.d, this.a, color);
    }

    //https://blog.hamaluik.ca/posts/simple-aabb-collision-using-minkowski-difference/
    fn minkowski(&self, other: &AABB) -> AABB {
        AABB {
            point: other.point,
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }

    fn point_collision(&self, point: Vec3) -> bool {
        let self_sides = self.sides();

        point.x > self_sides.left && point.x < self_sides.right &&
        point.y < self_sides.top && point.y > self_sides.bottom
    }

    /// This detects and corrects, which is why the return value is an option
    /// It's up to the user wether they wanna actually use the correction or not.
    pub fn static_static(&self, other: &AABB) -> Option<Vec3> {
        let minkowski = self.minkowski(&other);

        if !minkowski.point_collision(self.point) {
            return None;
        }
        
        let minkowski_sides = minkowski.sides();

        let mut minimum_distance = (self.point.x - minkowski_sides.left).abs();
        let mut bounds_point = Vec3::new(minkowski_sides.left, self.point.y, self.point.z);

        if (minkowski_sides.right - self.point.x).abs() < minimum_distance {
            minimum_distance = (minkowski_sides.right - self.point.x).abs();
            bounds_point = Vec3::new(minkowski_sides.right, self.point.y, self.point.z);
        }

        if (minkowski_sides.top - self.point.y).abs() < minimum_distance {
            minimum_distance = (minkowski_sides.top - self.point.y).abs();
            bounds_point = Vec3::new(self.point.x, minkowski_sides.top, self.point.z);
        }

        if (minkowski_sides.bottom - self.point.y).abs() < minimum_distance {
            // minimum_distance = (minkowski_sides.bottom - self.point.y).abs();
            bounds_point = Vec3::new(self.point.x, minkowski_sides.bottom, self.point.z);
        }
        
        Some(bounds_point)
    }

    /// x = true, y = false;
    fn clip_lines(&self, axis: bool, current: Vec3, next: Vec3) -> (bool, f32) {
        let self_sides = self.sides();

        let mut f_low;
        let mut f_high;
        
        match axis {
            true => {
                f_low = (self_sides.left - current.x) / (next.x - current.x);
                f_high = (self_sides.right - current.x) / (next.x - current.x);
            },
            false => {
                f_low = (self_sides.bottom - current.y) / (next.y - current.y);
                f_high = (self_sides.top - current.y) / (next.y - current.y);
            },
        }

        if f_high < f_low {
            std::mem::swap(&mut f_low, &mut f_high)
        }

        if f_high < 0.0 {
            return (false, 0.0);
        }

        if f_low > 1.0 {
            return (false, 0.0);
        }

        if f32::max(0.0, f_low) > f32::min(1.0, f_high) {
            return (false, 0.0);
        }
        
        (true, f_low)
    }

    pub fn intersect_line(&self, current: Vec3, next: Vec3) -> bool {
        if !self.clip_lines(true, current, next).0 {
            return false;
        }

        if !self.clip_lines(false, current, next).0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
