use bevy::prelude::*;

// https://www.youtube.com/watch?v=dD5-M-vUmls
#[derive(Component)]
pub struct CameraSettingsComponent {
  pub back: KeyCode,
  pub boost: KeyCode,
  pub boost_factor: f32,
  pub down: KeyCode,
  pub forward: KeyCode,
  pub left: KeyCode,
  pub mouse_sensitivity: f32,
  pub right: KeyCode,
  pub speed: f32,
  pub toggle_grab_cursor: KeyCode,
  pub up: KeyCode,
}

impl Default for CameraSettingsComponent {
  fn default() -> Self {
    Self {
      back: KeyCode::KeyS,
      boost: KeyCode::ShiftLeft,
      boost_factor: 4.,
      down: KeyCode::ControlLeft,
      forward: KeyCode::KeyW,
      left: KeyCode::KeyA,
      mouse_sensitivity: 0.000_12,
      right: KeyCode::KeyD,
      speed: 10.,
      toggle_grab_cursor: KeyCode::Escape,
      up: KeyCode::Space,
    }
  }
}
