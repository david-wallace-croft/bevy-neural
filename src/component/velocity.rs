use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct VelocityComponent {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
