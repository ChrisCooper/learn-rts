//use std::any::type_name;
mod tile;

use bevy::{prelude::{Plugin, App, Commands, Component, AssetServer, Res, Transform, Query}, sprite::SpriteBundle};
use tile::TileMaterials;

pub struct GMapPlugin;
impl Plugin for GMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TileMaterials>()
            .add_startup_system(create_inital_map)       
            .add_system(update_tiles);
    }
}

const MAP_SIZE: u32 = 40;
const TILE_SIZE_PX: u32 = 64;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct MineableResoure {
    type_name: String,
}


fn create_inital_map(mut commands: Commands, tile_materials: Res<TileMaterials>) {
    commands.spawn((Tile, MineableResoure { type_name: "empty".to_string() }))
    .insert(SpriteBundle {
        texture: tile_materials.empty_color.clone(),
        //transform: Transform::from_xyz(0.,0., 1.),
        ..Default::default()
    });
    println!("Spawned an empty tile");
}

fn update_tiles(commands: Commands, query: Query<(&Tile, &Transform)>) {
    for (tile, transform) in query.iter() {
        //println!("Tile");
    }
    
}