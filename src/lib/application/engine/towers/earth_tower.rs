use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{TargetSelection, Tower, TowerBase},
};

/// Tour de terre basique - dégâts faibles, attaque très rapide
#[derive(Debug, Clone)]
pub struct EarthBasicTower {
    base: TowerBase,
    upgrade_level: u32,
}

impl EarthBasicTower {
    pub fn positioned(position: Position) -> Self {
        Self {
            base: TowerBase {
                position,
                range: 3.0,
                element: Element::Earth,
                damage: 5.0,
                attacks_per_second: 4.0,
                aoe: false,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::Nearest,
            },
            upgrade_level: 0,
        }
    }
}

impl Tower for EarthBasicTower {
    fn position(&self) -> Position {
        self.base.position
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

    fn attack_damage(&self) -> f32 {
        self.base.damage
    }

    fn attack_speed(&self) -> f32 {
        self.base.attacks_per_second
    }

    fn last_attack_time(&self) -> f32 {
        self.base.last_attack
    }

    fn set_last_attack_time(&mut self, time: f32) {
        self.base.last_attack = time;
    }

    fn target_selection(&self) -> TargetSelection {
        self.base.target_selection.clone()
    }

    fn upgrade_level(&self) -> u32 {
        self.upgrade_level
    }

    fn upgrade_attack_speed(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.upgrade_level >= 5 {
            return false;
        }

        // Augmenter la vitesse d'attaque de 25% à chaque amélioration
        self.base.attacks_per_second *= 1.25;
        self.upgrade_level += 1;
        true
    }

    fn upgrade_damage(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.upgrade_level >= 5 {
            return false;
        }

        // Augmenter les dégâts de 20% à chaque amélioration
        self.base.damage *= 1.2;
        self.upgrade_level += 1;
        true
    }

    fn upgrade_range(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.upgrade_level >= 5 {
            return false;
        }

        // Augmenter la portée de 0.4 à chaque amélioration
        self.base.range += 0.4;
        self.upgrade_level += 1;
        true
    }
}
