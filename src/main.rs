mod kitty;
mod living;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba(0.0, 0.0,0.0,1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
