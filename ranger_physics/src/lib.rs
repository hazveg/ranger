use bevy::prelude::*;

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

    // Thank the lord I don't have to do any collision resolution... yet
    // Update: I envy you.
    // Update2: FUCK
    // Update3: fuck that resolution bullshit, i had to copy this code from https://www.youtube.com/watch?v=3vONlLYtHUE&t=0s, i sure as fuck am not torturing myself with that shit too.
    
    /// x = true, y = false;
    fn clip_lines(&self, axis: bool, current: Vec3, next: Vec3) -> bool {
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
            return false;
        }

        if f_low > 1.0 {
            return false;
        }

        if f32::max(0.0, f_low) > f32::min(1.0, f_high) {
            return false;
        }

        true
    }

    pub fn intersect_line(&self, current: Vec3, next: Vec3) -> bool {
        if !self.clip_lines(true, current, next) {
            return false;
        }

        if !self.clip_lines(false, current, next) {
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
