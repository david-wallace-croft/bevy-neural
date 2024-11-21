use super::component::acceleration::AccelerationComponent;
use super::component::position::PositionComponent;
use super::component::velocity::VelocityComponent;
use super::entity::ball::BallEntity;
use super::resource::debug_timer::DebugTimer;
use super::system::debug_printer::debug_printer_system;
use super::system::position_updater::update_position_system;
use super::system::velocity_updater::update_velocity_system;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;

#[derive(Default)]
pub struct Launcher {
  app: App,
}

impl Launcher {
  pub fn launch() {
    let _app_exit: AppExit = Launcher::default()
      .add_plugins()
      .add_startup_systems()
      .add_update_systems()
      .insert_resources()
      .run();
  }

  // private functions

  fn add_plugins(mut self) -> Self {
    self.app.add_plugins(DefaultPlugins);

    self
  }

  fn add_startup_systems(mut self) -> Self {
    let startup_systems = Launcher::spawn_entity;

    self.app.add_systems(Startup, startup_systems);

    self
  }

  fn add_update_systems(mut self) -> Self {
    let update_systems: SystemConfigs = (
      update_velocity_system,
      update_position_system,
      debug_printer_system,
    )
      .chain();

    self.app.add_systems(Update, update_systems);

    self
  }

  fn insert_resources(mut self) -> Self {
    let clear_color = ClearColor(Color::srgb(0.1, 0., 0.15));

    let _app: &mut App = self.app.insert_resource(clear_color);

    let ambient_light = AmbientLight {
      color: Default::default(),
      brightness: 1_000.,
    };

    let _app: &mut App = self.app.insert_resource(ambient_light);

    let timer = Timer::from_seconds(1., TimerMode::Repeating);

    let debug_timer = DebugTimer(timer);

    let _app: &mut App = self.app.insert_resource(debug_timer);

    self
  }

  fn run(mut self) -> AppExit {
    self.app.run()
  }

  fn spawn_entity(mut commands: Commands) {
    let ball_entity = BallEntity::new(0);

    let acceleration_component = AccelerationComponent::new(1e-6, 1e-6, 1e-6);

    let position_component = PositionComponent::default();

    let velocity_component = VelocityComponent::default();

    let component_bundle = (
      ball_entity,
      acceleration_component,
      position_component,
      velocity_component,
    );

    let _entity_commands: EntityCommands = commands.spawn(component_bundle);
  }
}
