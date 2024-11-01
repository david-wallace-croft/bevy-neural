use super::super::component::acceleration::AccelerationComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn update_velocity_system(
  mut query: Query<(&AccelerationComponent, &mut VelocityComponent)>,
  time: Res<Time>,
) {
  let delta_seconds: f64 = time.delta_seconds() as f64;

  for (acceleration, mut velocity) in query.iter_mut() {
    velocity.value += acceleration.value * delta_seconds;
  }
}
