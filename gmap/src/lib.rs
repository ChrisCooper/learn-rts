use std::any::type_name;

use bevy::prelude::{Plugin, App, Commands, Component};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct GMapPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct MineableResoure {
    type_name: String,
}

impl Plugin for GMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_inital_map)       
            .run()
    }
}

fn create_inital_map(mut commands: Commands) {
    commands.spawn((Tile, MineableResoure { type_name: "Wood".to_string() }));
    println!("Spawned a tile with wood");
}

