use super::super::component::camera_settings::CameraSettingsComponent;
use ::bevy::prelude::*;

// https://www.youtube.com/watch?v=dD5-M-vUmls
#[derive(Component)]
#[require(CameraSettingsComponent)]
pub struct FlyCameraComponent;
