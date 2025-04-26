use crate::domain::entities::monster::{Monster, Resistances};
use crate::domain::entities::position::Position;

pub fn goblin() -> Monster {
    Monster::new(
        "Gobelin".to_string(),
        50.0,
        Position::initial(),
        1.0,
        Resistances::default(),
        1,
    )
}

pub fn orc() -> Monster {
    Monster::new(
        "Orc".to_string(),
        100.0,
        Position::initial(),
        0.8,
        Resistances::default(),
        2,
    )
}
