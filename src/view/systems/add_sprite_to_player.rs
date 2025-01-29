use bevy::{prelude::*, sprite::Anchor};

use crate::{model::components::Player, view::ViewConstants};

pub fn add_sprite_to_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_player: Query<Entity, (With<Player>, Without<Sprite>)>,
) {
    for entity in q_player.iter() {
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
                index: 1,
            }),
            custom_size: Some(Vec2::splat(ViewConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        });
    }
}
