use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct AccelerationComponent {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
