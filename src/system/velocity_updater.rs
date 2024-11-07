use super::super::component::acceleration::AccelerationComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn update_velocity_system(
  mut query: Query<(&AccelerationComponent, &mut VelocityComponent)>,
  time: Res<Time>,
) {
  let delta_sec_f64: f64 = time.delta_secs_f64();

  for (acceleration, mut velocity) in query.iter_mut() {
    velocity.value += acceleration.value * delta_sec_f64;
  }
}
