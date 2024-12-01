use super::super::component::camera_settings::CameraSettingsComponent;
use super::super::component::fly_camera::FlyCameraComponent;
use ::bevy::input::mouse::MouseMotion;
use ::bevy::prelude::*;
use ::bevy::window::{CursorGrabMode, PrimaryWindow};
use ::std::f32::consts::PI;

// https://www.youtube.com/watch?v=dD5-M-vUmls
pub fn camera_rotator_system(
  mut event_reader: EventReader<MouseMotion>,
  mut transform_query: Query<
    (&CameraSettingsComponent, &mut Transform),
    With<FlyCameraComponent>,
  >,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  let delta: Vec2 = event_reader.read().map(|v| v.delta).sum();

  let Ok(window) = window_query.get_single() else {
    return;
  };

  if window.cursor_options.grab_mode != CursorGrabMode::Locked {
    return;
  };

  for (camera_settings, mut transform) in &mut transform_query {
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    let window_scale = window.height().min(window.width());

    pitch -=
      (camera_settings.mouse_sensitivity * delta.y * window_scale).to_radians();

    pitch = pitch.clamp(-PI / 2., PI / 2.);

    yaw -=
      (camera_settings.mouse_sensitivity * delta.x * window_scale).to_radians();

    transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
      * Quat::from_axis_angle(Vec3::X, pitch);
  }
}
