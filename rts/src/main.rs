use bevy::prelude::*;
use gmap;

fn main() {
    let num = 3;
    println!("Hello, world! {num} plus 5 is {}!", gmap::add(num, 5));


    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Prevents blurry sprites
        .add_plugin(gmap::GMapPlugin)
        .add_startup_system(setup_basics)
        .run();
}

fn setup_basics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
