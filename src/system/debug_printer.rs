use super::super::component::acceleration::AccelerationComponent;
use super::super::component::position::PositionComponent;
use super::super::component::velocity::VelocityComponent;
use ::bevy::prelude::*;

pub fn debug_printer_system(
  query: Query<(
    &AccelerationComponent,
    &PositionComponent,
    &VelocityComponent,
  )>,
  time: Res<Time>,
) {
  let (acceleration, position, velocity) = query.single();

  let delta_seconds: f64 = time.delta_secs() as f64;

  // TODO: print fixed number of digits

  println!(
    "{} {} {} {}",
    delta_seconds, acceleration.value, velocity.value, position.value
  );
}
