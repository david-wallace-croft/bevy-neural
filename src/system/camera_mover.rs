use super::super::component::camera_settings::CameraSettingsComponent;
use super::super::component::fly_camera::FlyCameraComponent;
use bevy::prelude::*;

// https://www.youtube.com/watch?v=dD5-M-vUmls
pub fn camera_mover_system(
  mut query: Query<
    (&CameraSettingsComponent, &mut Transform),
    With<FlyCameraComponent>,
  >,
  button_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  for (camera_settings, mut transform) in &mut query {
    let mut delta = Vec3::ZERO;
    
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

    let mut forward = transform.forward().as_vec3();

    forward.y = 0.;

    forward = forward.normalize();

    let mut right = transform.right().as_vec3();

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
