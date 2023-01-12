use bevy::prelude::*;


/// Initially from: https://github.com/StarArawn/bevy_ecs_tilemap
/// 
/// A tile position in the tilemap grid.
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}


#[derive(Resource)]
pub struct TileMaterials {
    pub empty_color: Handle<Image>,
}

impl FromWorld for TileMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let materials = world
            .get_resource::<AssetServer>()
            .unwrap();

        TileMaterials {
            empty_color:  materials.load("tile_empty_bright.png"),
        }
    }
}