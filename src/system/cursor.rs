use super::super::component::camera_settings::CameraSettingsComponent;
use ::bevy::prelude::*;
use ::bevy::window::CursorGrabMode;
use ::bevy::window::PrimaryWindow;

// https://www.youtube.com/watch?v=dD5-M-vUmls

pub fn cursor_grab_system(mut query: Query<&mut Window, With<PrimaryWindow>>) {
  let Ok(mut window) = query.single_mut() else {
    return;
  };

  window.cursor_options.grab_mode = CursorGrabMode::Locked;

  window.cursor_options.visible = false;
}

pub fn cursor_release_system(
  mut query: Query<&mut Window, With<PrimaryWindow>>
) {
  let Ok(mut window) = query.single_mut() else {
    return;
  };

  window.cursor_options.grab_mode = CursorGrabMode::None;

  window.cursor_options.visible = true;
}

pub fn cursor_toggle_system(
  button_input: Res<ButtonInput<KeyCode>>,
  camera_settings_query: Query<&CameraSettingsComponent>,
  mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
  let Ok(window) = window_query.single_mut() else {
    return;
  };

  let Ok(camera_settings) = camera_settings_query.single() else {
    return;
  };

  if button_input.just_pressed(camera_settings.toggle_grab_cursor) {
    match window.cursor_options.grab_mode {
      CursorGrabMode::None => cursor_grab_system(window_query),
      _ => cursor_release_system(window_query),
    }
  }
}
