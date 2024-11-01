use ::bevy::math::DVec3;
use ::bevy::prelude::*;

#[derive(Component, Default)]
pub struct PositionComponent {
  pub value: DVec3,
}
