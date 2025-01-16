use super::component::acceleration::AccelerationComponent;
use super::component::position::PositionComponent;
use super::component::velocity::VelocityComponent;
use super::entity::ball::BallEntity;
use super::resource::debug_timer::DebugTimer;
use super::system::ball_updater::update_ball_system;
// use super::system::camera_mover::camera_mover_system;
use super::system::debug_printer::debug_printer_system;
use super::system::position_updater::update_position_system;
use super::system::velocity_updater::update_velocity_system;
use bevy::app::PluginGroupBuilder;
use bevy::asset::RenderAssetUsages;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::render::render_resource::{
  Extent3d, TextureDimension, TextureFormat,
};

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
    let image_plugin = ImagePlugin::default_nearest();

    let plugin_group_builder: PluginGroupBuilder =
      DefaultPlugins.set(image_plugin);

    self.app.add_plugins(plugin_group_builder);

    self
  }

  fn add_startup_systems(mut self) -> Self {
    let startup_systems = (
      Launcher::spawn_balls,
      Launcher::spawn_camera,
      // Launcher::spawn_cubes,
    );

    self.app.add_systems(Startup, startup_systems);

    self
  }

  fn add_update_systems(mut self) -> Self {
    let update_systems: SystemConfigs = (
      // TODO: camera_mover_system,
      update_velocity_system,
      update_position_system,
      update_ball_system,
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

  fn spawn_balls(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
  ) {
    let image: Image = uv_debug_texture();

    // TODO: Figure out how to get an already created image
    let image_handle: Handle<Image> = image_assets.add(image);

    let base_color_texture: Option<Handle<Image>> = Some(image_handle);

    let standard_material = StandardMaterial {
      base_color_texture,
      ..default()
    };

    // TODO: Figure out how to get an already created material
    let debug_material: Handle<StandardMaterial> =
      standard_material_assets.add(standard_material);

    for index in 0..6 {
      // TODO: generate random values

      let acceleration_component = AccelerationComponent::new(1e-2, 1e-2, 1e-2);

      let position_component =
        PositionComponent::new(index as f32, index as f32, index as f32);

      let velocity_component = VelocityComponent::default();

      let asset = Sphere::default();

      // TODO: Figure out how to get an already created mesh
      let mesh: Handle<Mesh> = mesh_assets.add(asset);

      let debug_material_clone: Handle<StandardMaterial> =
        debug_material.clone();

      let material: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d::from(debug_material_clone);

      let mesh_clone: Handle<Mesh> = mesh.clone();

      let mesh: Mesh3d = Mesh3d::from(mesh_clone);

      let translation = Vec3::new(index as f32, index as f32, index as f32);

      let transform = Transform::from_translation(translation);

      let component_bundle = (
        BallEntity::new(index),
        acceleration_component,
        material,
        mesh,
        position_component,
        transform,
        velocity_component,
      );

      let entity_commands: EntityCommands = commands.spawn(component_bundle);

      let _entity: Entity = entity_commands.id();

      // TODO: Add Entity to a Vec and then set that as a Resource
    }
  }

  fn spawn_camera(mut commands: Commands) {
    let camera3d = Camera3d::default();

    let target = Vec3::new(0., 1., 0.);

    let transform =
      Transform::from_xyz(0., 7., 14.).looking_at(target, Vec3::Y);

    commands.spawn((camera3d, transform));
  }

  fn spawn_cubes(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
  ) {
    let asset = Cuboid::new(1., 1., 1.);

    let image: Image = uv_debug_texture();

    let image_handle: Handle<Image> = image_assets.add(image);

    let base_color_texture: Option<Handle<Image>> = Some(image_handle);

    let standard_material = StandardMaterial {
      base_color_texture,
      ..default()
    };

    let debug_material: Handle<StandardMaterial> =
      standard_material_assets.add(standard_material);

    let mesh: Handle<Mesh> = mesh_assets.add(asset);

    // https://youtu.be/IHRI01Oqj60?si=8VFt_g1qYgw2vokQ
    for x in -10..10 {
      for z in -10..10 {
        let debug_material_clone: Handle<StandardMaterial> =
          debug_material.clone();

        let material: MeshMaterial3d<StandardMaterial> =
          MeshMaterial3d::from(debug_material_clone);

        let mesh_clone: Handle<Mesh> = mesh.clone();

        let mesh3d = Mesh3d::from(mesh_clone);

        let translation = Vec3::new((2 * x) as f32, 0., (2 * z) as f32);

        let transform = Transform::from_translation(translation);

        let bundle = (Shape, material, mesh3d, transform);

        let _entity_commands: EntityCommands<'_>  = commands.spawn(bundle);
      }
    }
  }
}

// https://bevyengine.org/examples/3d-rendering/3d-shapes/
fn uv_debug_texture() -> Image {
  const TEXTURE_SIZE: usize = 8;

  let mut palette: [u8; 32] = [
    255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102,
    255, 102, 255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102,
    255, 255,
  ];

  let mut texture_data: [u8; 256] = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

  for y in 0..TEXTURE_SIZE {
    let offset = TEXTURE_SIZE * y * 4;

    texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);

    palette.rotate_right(4);
  }

  let size = Extent3d {
    width: TEXTURE_SIZE as u32,
    height: TEXTURE_SIZE as u32,
    depth_or_array_layers: 1,
  };

  let dimension: TextureDimension = TextureDimension::D2;

  let pixel: &[u8; 256] = &texture_data;

  let format: TextureFormat = TextureFormat::Rgba8UnormSrgb;
  
  let asset_usage: RenderAssetUsages = RenderAssetUsages::RENDER_WORLD;

  Image::new_fill(
    size,
    dimension,
    pixel,
    format,
    asset_usage,
  )
}

#[derive(Component)]
pub struct Shape;
