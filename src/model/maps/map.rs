use bevy::{prelude::*, utils::HashMap};
use brt::grid::Grid;

use crate::model::components::{Description, Position, TerrainType};

#[derive(Reflect, Clone)]
pub struct Map {
    pub size: (u32, u32),

    pub terrain: Grid<Entity>,
    pub actors: HashMap<Position, Entity>,
}

impl Map {
    pub fn new(commands: &mut Commands, size: (u32, u32)) -> Self {
        let terrain = Grid::new_fn(size, |_index, (x, y)| {
            let (tile_type, tile_description) =
                if x == 0 || y == 0 || x == size.0 as i32 - 1 || y == size.1 as i32 - 1 {
                    (TerrainType::Wall, Description::new("Wall"))
                } else {
                    (TerrainType::Floor, Description::new("Floor"))
                };
            commands
                .spawn((Position::new(x, y), tile_type, tile_description))
                .id()
        });
        let actors = HashMap::new();

        Self {
            size,
            terrain,
            actors,
        }
    }

    pub fn get_terrain(&self, position: Position) -> Option<Entity> {
        self.terrain.get(position.into()).copied()
    }

    pub fn get_actor(&self, position: Position) -> Option<Entity> {
        self.actors.get(&position).copied()
    }

    pub fn set_actor(&mut self, position: Position, actor: Option<Entity>) {
        if let Some(actor) = actor {
            self.actors.insert(position, actor);
        } else {
            self.actors.remove(&position);
        }
    }
}
