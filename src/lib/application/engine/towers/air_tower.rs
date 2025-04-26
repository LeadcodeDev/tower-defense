use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{Tower, TowerBase},
};

/// Tour d'air basique - faibles dégâts, zone d'effet, attaque rapide
#[derive(Debug, Clone)]
pub struct AirBasicTower {
    base: TowerBase,
}

impl AirBasicTower {
    pub fn positioned(position: Position) -> Self {
        Self {
            base: TowerBase {
                position,
                range: 2.5,
                element: Element::Air,
                damage: 4.0,
                attacks_per_second: 3.0,
                aoe: true,
                behavior: TowerBehavior::Basic,
            },
        }
    }
}

impl Tower for AirBasicTower {
    fn position(&self) -> &Position {
        &self.base.position
    }

    fn range(&self) -> f32 {
        self.base.range
    }

    fn get_element(&self) -> Element {
        self.base.element
    }

    fn damage(&self) -> f32 {
        self.base.damage
    }

    fn attacks_per_second(&self) -> f32 {
        self.base.attacks_per_second
    }

    fn is_aoe(&self) -> bool {
        self.base.aoe
    }

    fn behavior(&self) -> &TowerBehavior {
        &self.base.behavior
    }
}
