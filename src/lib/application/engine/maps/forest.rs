use crate::{
    application::engine::monsters::basics::{goblin, orc},
    domain::entities::{
        map::{Map, TerrainType},
        monster::{Monster, Resistances},
        position::Position,
    },
};

pub fn forest_map() -> Map {
    Map::new(
        "Chemin forestier".to_string(),
        vec![
            Position::new(0, 7),
            Position::new(5, 7),
            Position::new(5, 3),
            Position::new(10, 3),
            Position::new(10, 12),
            Position::new(15, 12),
            Position::new(15, 7),
            Position::new(20, 7),
        ],
        20,
        15,
        TerrainType::Forest,
        vec![goblin(), orc()],
    )
}
