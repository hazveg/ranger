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

    pub fn is_moving(&self) -> bool {
        //println!("{}, {}, {}", self.movement.x, self.movement.y, self.movement.z);
        self.movement.x != 0.0 && self.movement.y != 0.0 && self.movement.z != 0.0
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

    pub fn point_collision(&self, point: Vec3) -> bool {
        let self_sides = self.sides();

        point.x > self_sides.left && point.x < self_sides.right &&
        point.y < self_sides.top && point.y > self_sides.bottom
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
    
    /// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
    ///
    /// It's a less stupid version of the previous ray casting code.
    ///     |   /
    ///     |  /|
    /// ----+-x-+----
    ///     |/  |
    /// ----x---+----
    ///    /|   |
    ///   / |   |
    /// It'll get you the Xs in the illustration and return the lesser of the two when they collide
    /// Honestly if you need a good explanation, just refer to the article at the very start of
    /// this documentation
    pub fn raycast(&self, origin: Vec3, destination: Vec3) -> Option<f32> {
        // we get the sides and the normalized direction of our movement vector
        let self_sides = self.sides();
        let direction = destination.normalize_or_zero();
        
        // we solve for the point in time where our ray intersects left/right for x and bottom/top
        // for y respectively
        let t_min_x = (self_sides.left - origin.x) / direction.x;
        let t_max_x = (self_sides.right - origin.x) / direction.x;
        let t_min_y = (self_sides.bottom - origin.y) / direction.y;
        let t_max_y = (self_sides.top - origin.y) / direction.y;
        
        // with this we get the maximum t_min and the minimum t_max
        let t_min = (t_min_x.min(t_max_x)).max(t_min_x.min(t_max_x));
        let t_max = (t_min_x.max(t_max_x)).min(t_min_y.max(t_max_y));
        
        // run some checks on the values
        if t_max < 0.0 {
            return None;
        }

        if t_min > t_max {
            return None;
        }

        if t_min < 0.0 {
            return Some(t_max);
        }

        return Some(t_min);
    }

    /// Here for moving/static and static/static, given a point; it returns a point which is
    /// derived from the minimum distance required to get out of the bounding box.
    ///
    /// Said bounding box is intended to be a minkowski one.
    ///
    /// +-----------+
    /// |           |
    /// |--*        |
    /// |           |
    /// +-----------+
    fn get_bounds_point_from_minimum_distance(&self, point: Vec3) -> Vec3 {
        let minkowski_sides = self.sides();

        let mut minimum_distance = (point.x - minkowski_sides.left).abs();
        let mut bounds_point = Vec3::new(minkowski_sides.left, point.y, point.z);

        if (minkowski_sides.right - point.x).abs() < minimum_distance {
            minimum_distance = (minkowski_sides.right - point.x).abs();
            bounds_point = Vec3::new(minkowski_sides.right, point.y, point.z);
        }

        if (minkowski_sides.top - point.y).abs() < minimum_distance {
            minimum_distance = (minkowski_sides.top - point.y).abs();
            bounds_point = Vec3::new(point.x, minkowski_sides.top, point.z);
        }

        if (point.y - minkowski_sides.bottom).abs() < minimum_distance {
            //minimum_distance = (minkowski_sides.bottom - point.y).abs();
            bounds_point = Vec3::new(point.x, minkowski_sides.bottom, point.z);
        }
        
        bounds_point
    }

    /// This detects and corrects, which is why the return value is an option
    /// It's up to the user wether they wanna actually use the correction or not.
    pub fn static_static(&self, other: &AABB) -> Option<Vec3> {
        let minkowski = self.minkowski(other);

        if !minkowski.point_collision(self.point) {
            return None;
        }
        
        Some(minkowski.get_bounds_point_from_minimum_distance(self.point))
    }

    fn dynamic_static(&self, my_path: &Path, other: &AABB) -> bool {
        // minkowski is created at the other box's position. we then raycast our movement vector
        // and if we get something back from the raycast, we'll collide.
        let minkowski = self.minkowski(other);
        if let Some(_) = minkowski.raycast(self.point, my_path.movement) {
            return true;
        }
        println!("no collision ):");

        false
    }

    pub fn is_colliding(first_aabb: &AABB, first_path: &Path, second_aabb: &AABB, second_path: &Path) -> bool {
        println!("first: {}, second: {}", first_path.movement, second_path.movement);
        match (first_path.is_moving(), second_path.is_moving()) {
            (true, false) => {
                return first_aabb.dynamic_static(first_path, second_aabb);
            },
            (false, true) => {
                return second_aabb.dynamic_static(first_path, first_aabb);
            },
            (true, true) => {
                // probably won't bother doing this, ever.
                return false;
            },
            (false, false) => {
                //println!("{}, {}", first_path.movement, second_path.movement);
                /*if let Some(_) = first_aabb.static_static(second_aabb) {
                    return true;
                }*/
            },
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
