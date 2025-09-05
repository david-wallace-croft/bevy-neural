use ::bevy::prelude::*;

#[derive(Component, Default)]
pub struct PositionComponent {
  pub value: Vec3,
}

impl PositionComponent {
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let value: Vec3 = Vec3::new(x, y, z);

    Self {
      value,
    }
  }
}
