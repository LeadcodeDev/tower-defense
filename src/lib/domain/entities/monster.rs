use super::{element::Element, map::Map, position::Position};

#[derive(Debug, Clone)]
pub struct Resistances {
    pub fire: f32,
    pub water: f32,
    pub earth: f32,
    pub air: f32,
}

impl Resistances {
    pub fn new(fire: f32, water: f32, earth: f32, air: f32) -> Self {
        Self {
            fire,
            water,
            earth,
            air,
        }
    }

    // Obtenir la résistance à un élément spécifique
    pub fn get_resistance(&self, element: &Element) -> f32 {
        match element {
            Element::Fire => self.fire,
            Element::Water => self.water,
            Element::Earth => self.earth,
            Element::Air => self.air,
            Element::Neutral => 0.0,
            Element::Lightning => 0.0,
            Element::Ice => 0.0,
            Element::Poison => 0.0,
        }
    }

    // Défini une résistance par défaut pour les monstres
    pub fn default() -> Self {
        Self {
            fire: 0.0,
            water: 0.0,
            earth: 0.0,
            air: 0.0,
        }
    }

    // Calcule le facteur final de dégâts en tenant compte des résistances et vulnérabilités
    pub fn damage_factor(&self, attack_element: &Element) -> f32 {
        let resistance = self.get_resistance(attack_element);
        1.0 - resistance
    }
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub hp: f32,
    pub position: Position,
    pub movement_speed: f32, // Cases par seconde (larger = faster)
    pub waypoint_idx: usize,
    pub resistances: Resistances,
    pub damage_to_player: u32,
    pub distance_moved: f32, // Distance accumulée pour les mouvements partiels
    pub spawn_delay: f32,    // Délai avant apparition (en secondes)
    pub active: bool,        // Indique si le monstre est actif dans la vague
}

impl Monster {
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0 && self.active
    }

    pub fn update_spawn_status(&mut self, delta_time: f32) -> bool {
        if !self.active {
            self.spawn_delay -= delta_time;
            if self.spawn_delay <= 0.0 {
                self.active = true;
                return true; // Le monstre vient d'être activé
            }
        }
        false
    }

    // Déplacer le monstre en fonction de sa vitesse en cases/seconde et du temps écoulé
    pub fn advance(&mut self, map: &Map, delta_time: f32) {
        // Ne rien faire si le monstre n'est pas encore actif
        if !self.active {
            return;
        }

        if self.waypoint_idx >= map.waypoints.len() {
            return;
        }

        let target = map.waypoints[self.waypoint_idx];
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;

        // Si le monstre a atteint le waypoint cible
        if dx == 0 && dy == 0 {
            self.waypoint_idx += 1;
            return;
        }

        // Calculer la distance totale au waypoint cible
        let dist = ((dx * dx + dy * dy) as f32).sqrt();

        // Utiliser directement movement_speed comme cases par seconde
        // et l'ajuster en fonction du type de terrain
        let terrain_modifier = map.terrain_type.speed_modifier();
        let cases_per_second = self.movement_speed * terrain_modifier;

        // Calculer le déplacement pour ce frame
        let distance_this_frame = cases_per_second * delta_time;

        // Ajouter à la distance accumulée
        self.distance_moved += distance_this_frame;

        // Calculer combien de cases entières à déplacer
        let steps = self.distance_moved.floor() as i32;
        self.distance_moved -= steps as f32;

        if steps > 0 {
            // Calculer le déplacement en x et y
            let step_x = ((dx as f32) / dist * steps as f32).round() as i32;
            let step_y = ((dy as f32) / dist * steps as f32).round() as i32;

            // Appliquer le déplacement
            self.position.x += step_x;
            self.position.y += step_y;

            // Vérifier si on a atteint ou dépassé le waypoint
            let new_dx = target.x - self.position.x;
            let new_dy = target.y - self.position.y;

            // Si on a dépassé le waypoint ou changé de direction, aller directement au waypoint
            if new_dx * dx <= 0 && new_dy * dy <= 0 {
                self.position = target;
                self.waypoint_idx += 1;
            }
        }
    }

    pub fn reached_goal(&self, map: &Map) -> bool {
        self.active && self.waypoint_idx >= map.waypoints.len()
    }

    pub fn set_movement_speed(&mut self, cases_per_second: f32) {
        // Plus grand = plus rapide, mais avec un minimum pour éviter une vitesse nulle
        self.movement_speed = cases_per_second.max(0.1); // minimum 0.1 case par seconde
    }

    pub fn new(
        name: String,
        hp: f32,
        position: Position,
        movement_speed: f32,
        resistances: Resistances,
        damage_to_player: u32,
    ) -> Self {
        Self {
            name,
            hp,
            position,
            movement_speed,
            waypoint_idx: 1,
            resistances,
            damage_to_player,
            distance_moved: 0.0,
            spawn_delay: 0.0, // Par défaut, pas de délai
            active: true,     // Par défaut, actif immédiatement
        }
    }

    pub fn with_spawn_delay(mut self, delay: f32) -> Self {
        self.spawn_delay = delay;
        self.active = delay <= 0.0; // Actif seulement si pas de délai
        self
    }
}
