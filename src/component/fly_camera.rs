use super::super::component::camera_settings::CameraSettingsComponent;
use super::super::system::cursor::cursor_grab_system;
use crate::system::camera::camera_rotator_system;
use crate::system::camera::camera_translator_system;
use crate::system::cursor::cursor_toggle_system;
use ::bevy::prelude::*;

// https://www.youtube.com/watch?v=dD5-M-vUmls

#[derive(Component)]
#[require(CameraSettingsComponent)]
pub struct FlyCameraComponent;

impl Plugin for FlyCameraComponent {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .add_systems(Startup, cursor_grab_system)
      .add_systems(Update, cursor_toggle_system)
      .add_systems(
        Update,
        (camera_rotator_system, camera_translator_system).chain(),
      );
  }
}
