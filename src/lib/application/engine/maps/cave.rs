use crate::{
    application::engine::monsters::basics::{goblin, orc},
    domain::entities::{map::Map, position::Position},
};

pub struct CaveMap;

impl CaveMap {
    pub fn new() -> Map {
        let start_position = Position::new(0, 5);

        Map::new(
            "Caverne Profonde".to_string(),
            "Une caverne profonde et sombre".to_string(),
            "🌵".to_string(),
            "🏠".to_string(),
            vec![
                start_position,
                Position::new(3, 5),
                Position::new(3, 10),
                Position::new(7, 10),
                Position::new(7, 5),
                Position::new(12, 5),
                Position::new(12, 12),
                Position::new(18, 12),
                Position::new(18, 7),
                Position::new(20, 7),
                Position::new(20, 12),
                Position::new(40, 17),
            ],
            40, // largeur de la carte
            17, // hauteur de la carte
            vec![goblin(start_position), orc(start_position)],
            None,
        )
    }
}
