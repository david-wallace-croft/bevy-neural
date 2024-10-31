use ::bevy::math::DVec3;
use ::bevy::prelude::Component;

#[derive(Component, Default)]
pub struct AccelerationComponent(pub DVec3);
