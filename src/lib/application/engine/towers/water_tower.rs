use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{Tower, TowerBase},
};

/// Tour d'eau basique - dégâts modérés, vitesse d'attaque moyenne
#[derive(Debug, Clone)]
pub struct WaterBasicTower {
    base: TowerBase,
}

impl WaterBasicTower {
    pub fn positioned(position: Position) -> Self {
        Self {
            base: TowerBase {
                position,
                range: 3.5,
                element: Element::Water,
                damage: 10.0,
                attacks_per_second: 1.0,
                aoe: false,
                behavior: TowerBehavior::Basic,
            },
        }
    }
}

impl Tower for WaterBasicTower {
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
