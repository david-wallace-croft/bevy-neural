use super::super::component::position::PositionComponent;
use super::super::entity::ball::BallEntity;
use ::bevy::prelude::*;

pub fn update_ball_system(
  mut query: Query<(&PositionComponent, &mut Transform), With<BallEntity>>
) {
  for (position, mut transform) in query.iter_mut() {
    transform.translation = position.value;
  }
}
