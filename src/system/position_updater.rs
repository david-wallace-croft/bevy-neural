use super::super::component::position::PositionComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn update_position_system(
  mut query: Query<(&mut PositionComponent, &VelocityComponent)>,
  time: Res<Time>,
) {
  let delta_seconds: f64 = time.delta_seconds() as f64;

  for (mut position, velocity) in query.iter_mut() {
    position.value += velocity.value * delta_seconds;
  }
}
