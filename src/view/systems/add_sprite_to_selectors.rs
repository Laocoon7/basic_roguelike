use bevy::{prelude::*, sprite::Anchor};

use crate::{controller::components::Selector, view::ViewConstants};

pub fn add_sprite_to_selectors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_selectors: Query<Entity, (With<Selector>, Without<Sprite>)>,
) {
    for entity in q_selectors.iter() {
        commands.entity(entity).insert(Sprite {
            image: asset_server.load("selection.png"),
            texture_atlas: Some(TextureAtlas {
                layout: asset_server.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    2,
                    1,
                    None,
                    None,
                )),
                index: 0,
            }),
            // color: todo!(), // TODO: Seperate system to check actors/items/features at position and color accordingly
            custom_size: Some(Vec2::splat(ViewConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        });
    }
}
