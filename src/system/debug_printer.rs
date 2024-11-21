use super::super::component::acceleration::AccelerationComponent;
use super::super::component::position::PositionComponent;
use super::super::component::velocity::VelocityComponent;
use super::super::entity::ball::BallEntity;
use ::bevy::prelude::*;

pub fn debug_printer_system(
  query: Query<(
    &BallEntity,
    &AccelerationComponent,
    &PositionComponent,
    &VelocityComponent,
  )>,
  time: Res<Time>,
) {
  let (ball, acceleration, position, velocity) = query.single();

  let delta_seconds: f64 = time.delta_secs_f64();

  // TODO: print fixed number of digits

  println!(
    "{} {:.6} {:.6} {:.6} {:.6}",
    ball.id, delta_seconds, acceleration.value, velocity.value, position.value
  );
}
