use crate::{
    application::engine::monsters::basics::{goblin, orc},
    domain::entities::{map::Map, position::Position},
};

pub struct ForestMap;

impl ForestMap {
    pub fn new() -> Map {
        let start_position = Position::new(0, 7);

        Map::new(
            "Forêt Enchantée".to_string(),
            "Une forêt dense et mystérieuse".to_string(),
            "🌵".to_string(),
            "🚀".to_string(),
            vec![
                start_position,
                Position::new(5, 7),
                Position::new(5, 3),
                Position::new(10, 3),
                Position::new(10, 12),
                Position::new(15, 12),
                Position::new(15, 7),
                Position::new(20, 7),
            ],
            20, // largeur de la carte
            20, // hauteur de la carte
            vec![goblin(start_position), orc(start_position)],
            None,
        )
    }
}
