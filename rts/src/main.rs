use bevy::prelude::*;
use gmap;

fn main() {
    println!("Launching app");

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Prevents blurry sprites
        .add_plugin(gmap::GMapPlugin)
        .add_startup_system(setup_basics)
        .run();
}

fn setup_basics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
