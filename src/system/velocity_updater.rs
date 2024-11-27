use super::super::component::acceleration::AccelerationComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn update_velocity_system(
  mut query: Query<(&AccelerationComponent, &mut VelocityComponent)>,
  time: Res<Time>,
) {
  let delta_sec: f32 = time.delta_secs();

  for (acceleration, mut velocity) in query.iter_mut() {
    velocity.value += acceleration.value * delta_sec;
  }
}
