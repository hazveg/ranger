use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Target {
    pub point: Option<Vec3>,
}

impl Target {
    pub fn new(point: Option<Vec3>) -> Self {
        Target { point }
    }

    pub fn set_target(&mut self, point: Vec3) {
        self.point = Some(point);
    }

    pub fn remove_target(&mut self) {
        self.point = None;
    }

    pub fn has_target(&self) -> bool {
        self.point.is_some()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
