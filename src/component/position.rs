use ::bevy::math::DVec3;
use ::bevy::prelude::*;

#[derive(Component, Default)]
pub struct PositionComponent {
  pub value: DVec3,
}

impl PositionComponent {
  pub fn new(
    x: f64,
    y: f64,
    z: f64,
  ) -> Self {
    let value = DVec3::new(x, y, z);

    Self {
      value,
    }
  }
}
