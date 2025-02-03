use bevy::{prelude::*, sprite::Anchor};

use crate::{model::components::TerrainType, view::ViewConstants};

pub fn add_sprite_to_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_tiles: Query<(Entity, &TerrainType), Without<Sprite>>,
) {
    for (entity, tile_type) in q_tiles.iter() {
        commands.entity(entity).insert(Sprite {
            image: asset_server.load("terminal_32x32.png"),
            texture_atlas: Some(TextureAtlas {
                layout: asset_server.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    16,
                    16,
                    None,
                    None,
                )),
                index: match tile_type {
                    TerrainType::Floor => 250,
                    TerrainType::Wall => 35,
                },
            }),
            custom_size: Some(Vec2::splat(ViewConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        });
    }
}
