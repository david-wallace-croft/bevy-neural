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
  let delta: Vec2 = event_reader.read().map(|v: &MouseMotion| v.delta).sum();

  let Ok(window) = window_query.single() else {
    return;
  };

  if window.cursor_options.grab_mode != CursorGrabMode::Locked {
    return;
  };

  for (camera_settings, mut transform) in &mut transform_query {
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    let window_scale: f32 = window.height().min(window.width());

    pitch -=
      (camera_settings.mouse_sensitivity * delta.y * window_scale).to_radians();

    pitch = pitch.clamp(-PI / 2., PI / 2.);

    yaw -=
      (camera_settings.mouse_sensitivity * delta.x * window_scale).to_radians();

    transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
      * Quat::from_axis_angle(Vec3::X, pitch);
  }
}

pub fn camera_translator_system(
  mut query: Query<
    (&CameraSettingsComponent, &mut Transform),
    With<FlyCameraComponent>,
  >,
  button_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  for (camera_settings, mut transform) in &mut query {
    let mut delta: Vec3 = Vec3::ZERO;

    if button_input.pressed(camera_settings.forward) {
      delta.z += 1.;
    }

    if button_input.pressed(camera_settings.back) {
      delta.z -= 1.;
    }

    if button_input.pressed(camera_settings.left) {
      delta.x -= 1.;
    }

    if button_input.pressed(camera_settings.right) {
      delta.x += 1.;
    }

    let mut forward: Vec3 = transform.forward().as_vec3();

    forward.y = 0.;

    forward = forward.normalize();

    let mut right: Vec3 = transform.right().as_vec3();

    right.y = 0.;

    right = right.normalize();

    if button_input.pressed(camera_settings.up) {
      delta.y += 1.;
    }

    if button_input.pressed(camera_settings.down) {
      delta.y -= 1.;
    }

    if button_input.pressed(camera_settings.boost) {
      delta *= camera_settings.boost_factor;
    }

    let delta: Vec3 = (forward * delta.z + right * delta.x + Vec3::Y * delta.y)
      * time.delta_secs()
      * camera_settings.speed;

    if !delta.is_nan() {
      transform.translation += delta;
    }
  }
}
