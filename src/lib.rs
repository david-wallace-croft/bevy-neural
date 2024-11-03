use self::component::acceleration::AccelerationComponent;
use self::component::position::PositionComponent;
use self::component::velocity::VelocityComponent;
use self::system::debug_printer::debug_printer_system;
use self::system::position_updater::update_position_system;
use self::system::velocity_updater::update_velocity_system;
use ::bevy::ecs::schedule::SystemConfigs;
use ::bevy::ecs::system::EntityCommands;
use ::bevy::prelude::*;

mod component;
mod system;

pub fn launch() {
  let mut app = App::new();

  let _app: &mut App = app.add_plugins(DefaultPlugins);

  let startup_systems = spawn_entity;

  let _app: &mut App = app.add_systems(Startup, startup_systems);

  let update_systems: SystemConfigs = (
    update_velocity_system,
    update_position_system,
    debug_printer_system,
  )
    .chain();

  let _app: &mut App = app.add_systems(Update, update_systems);

  let _app_exit: AppExit = app.run();
}

fn spawn_entity(mut commands: Commands) {
  let acceleration_component = AccelerationComponent::default();

  let position_component = PositionComponent::default();

  let velocity_component = VelocityComponent::default();

  let component_bundle = (
    acceleration_component,
    position_component,
    velocity_component,
  );

  let _entity_commands: EntityCommands = commands.spawn(component_bundle);
}
