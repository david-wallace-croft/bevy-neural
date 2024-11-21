use super::super::component::acceleration::AccelerationComponent;
use super::super::component::position::PositionComponent;
use super::super::component::velocity::VelocityComponent;
use super::super::entity::ball::BallEntity;
use super::super::resource::debug_timer::DebugTimer;
use ::bevy::prelude::*;
use ::std::time::Duration;

pub fn debug_printer_system(
  mut debug_timer: ResMut<DebugTimer>,
  query: Query<(
    &BallEntity,
    &AccelerationComponent,
    &PositionComponent,
    &VelocityComponent,
  )>,
  time: Res<Time>,
) {
  let time_delta: Duration = time.delta();

  let timer: &Timer = debug_timer.0.tick(time_delta);

  if !timer.just_finished() {
    return;
  }

  let (ball, acceleration, position, velocity) = query.single();

  let delta_seconds: f64 = time.delta_secs_f64();

  // TODO: print fixed number of digits

  println!(
    "{} {:.6} {:.6} {:.6} {:.6}",
    ball.id, delta_seconds, acceleration.value, velocity.value, position.value
  );
}
