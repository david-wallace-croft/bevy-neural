use ::bevy::prelude::*;

#[derive(Component)]
pub struct BallEntity {
  pub id: usize,
}

impl BallEntity {
  pub fn new(id: usize) -> Self {
    Self {
      id,
    }
  }
}
