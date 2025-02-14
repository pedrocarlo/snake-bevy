use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use itertools::iproduct;

use crate::SpriteSheet;

pub(crate) const WORLD_W: usize = 30;
pub(crate) const WORLD_H: usize = 30;
pub(crate) const TILE_W: usize = 16;
pub(crate) const TILE_H: usize = 16;

pub enum SpriteParts {
    SnakeHead,
    SnakeBody,
    Empty,
    Grass,
    Food,
    Wall,
}

impl From<usize> for SpriteParts {
    fn from(value: usize) -> Self {
        use SpriteParts::*;

        match value {
            0 => SnakeHead,
            1 => SnakeBody,
            2 => Empty,
            3 => Grass,
            4 => Food,
            5 => Wall,
            x => panic!("Unknown TileTextureIndex {x}"),
        }
    }
}

impl From<SpriteParts> for usize {
    fn from(value: SpriteParts) -> Self {
        use SpriteParts::*;
        match value {
            SnakeHead => 0,
            SnakeBody => 1,
            Empty => 2,
            Grass => 3,
            Food => 4,
            Wall => 5,
        }
    }
}

impl From<u32> for SpriteParts {
    fn from(value: u32) -> Self {
        use SpriteParts::*;

        match value {
            0 => SnakeHead,
            1 => SnakeBody,
            2 => Empty,
            3 => Grass,
            4 => Food,
            5 => Wall,
            x => panic!("Unknown TileTextureIndex {x}"),
        }
    }
}

impl From<SpriteParts> for u32 {
    fn from(value: SpriteParts) -> Self {
        use SpriteParts::*;
        match value {
            SnakeHead => 0,
            SnakeBody => 1,
            Empty => 2,
            Grass => 3,
            Food => 4,
            Wall => 5,
        }
    }
}

impl From<TileTextureIndex> for SpriteParts {
    fn from(value: TileTextureIndex) -> Self {
        value.0.into()
    }
}

impl From<SpriteParts> for TileTextureIndex {
    fn from(value: SpriteParts) -> Self {
        TileTextureIndex(value.into())
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);

        app.add_systems(Startup, gen_terrain);
    }
}

#[derive(Component)]
#[require(Collider)]
pub struct Wall;

#[derive(Component)]
#[require(Collider)]
pub struct Grass;

#[derive(Component)]
pub struct Empty;

#[derive(Component, Default)]
#[require(Collider)]
pub struct Food;

fn gen_terrain(mut commands: Commands, texture: Res<SpriteSheet>) {
    let map_size = TilemapSize {
        x: WORLD_W as u32,
        y: WORLD_H as u32,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    let mut tile_storage = TileStorage::empty(map_size);

    let origin = TilePos { x: 0, y: 0 };
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos {
                    x: origin.x + x,
                    y: origin.y + y,
                };

                let tile_entity = parent
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id,
                            texture_index: SpriteParts::Empty.into(),
                            ..Default::default()
                        },
                        Empty,
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    let square_coords = box_coords(URect::from_corners(
        UVec2::new(0, 0),
        UVec2 {
            x: map_size.x - 1,
            y: map_size.y - 1,
        },
    ));

    commands.entity(tilemap_id.0).with_children(|parent| {
        for UVec2 { x, y } in square_coords {
            let tile_pos = TilePos { x, y };

            let tile_entity = parent
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: SpriteParts::Wall.into(),
                        // color: TileColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                        ..Default::default()
                    },
                    Wall,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    });

    let tile_size = TilemapTileSize {
        x: TILE_W as f32,
        y: TILE_H as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn box_coords(rect: URect) -> Vec<UVec2> {
    let start = rect.min;
    let end = rect.max;

    // Easy to parallel iter with rayon
    iproduct!(start.x..=end.x, start.y..=end.y)
        .filter_map(|(x, y)| {
            // If inside this square then do not return the coordinate
            match (
                x >= start.x + 1 && x <= end.x - 1,
                y >= start.y + 1 && y <= end.y - 1,
            ) {
                (true, true) => None,
                _ => Some(UVec2 { x, y }),
            }
        })
        .collect()
}
