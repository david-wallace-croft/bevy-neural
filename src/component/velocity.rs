use ::bevy::prelude::*;

#[derive(Component, Default)]
pub struct VelocityComponent {
  pub value: Vec3,
}

impl VelocityComponent {
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let value = Vec3::new(x, y, z);

    Self {
      value,
    }
  }
}
