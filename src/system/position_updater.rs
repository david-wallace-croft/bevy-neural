use super::super::component::position::PositionComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn update_position_system(
  mut query: Query<(&mut PositionComponent, &VelocityComponent)>,
  time: Res<Time>,
) {
  let delta_secs: f32 = time.delta_secs();

  for (mut position, velocity) in query.iter_mut() {
    position.value += velocity.value * delta_secs;
  }
}
