use std::any::type_name;

use bevy::{prelude::{Plugin, App, Commands, Component, AssetServer, Res}, sprite::SpriteBundle};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

const MAP_SIZE: u32 = 40;
const TILE_SIZE_PX: u32 = 64;

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
            .add_startup_system(load_assets)       
            .run()
    }
}

fn create_inital_map(mut commands: Commands) {
    commands.spawn((Tile, MineableResoure { type_name: "empty".to_string() }));
    println!("Spawned an empty tile");
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tile_empty.png"),
        ..Default::default()
    });
}



tile
-x,y
-resources
-is_base