use bevy::prelude::*;

fn main() {
  let mut app = App::new();

  let _app: &mut App = app.add_systems(Update, my_first_system);

  let _app_exit: AppExit = app.run();
}

fn my_first_system() {
  println!("Hello, World!");
}
