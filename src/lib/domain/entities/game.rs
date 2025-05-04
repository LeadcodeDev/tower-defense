use std::{
    collections::VecDeque,
    time::{Duration, Instant, SystemTime},
};

use rand::{Rng, rng};

use crate::{
    application::engine::towers::{
        air_tower::AirTower, basic_tower::BasicTower, earth_tower::EarthTower,
        fire_tower::FireTower, sentinel_tower::SentinelTower,
    },
    infrastructure::ui::{app::App, notifications::Notifier},
};

use super::{
    map::Map,
    monster::Monster,
    position::Position,
    tower::{Tower, UpgradeType},
    wave::Wave,
};

/// Structure représentant un log d'événement du jeu
#[derive(Debug, Clone)]
pub struct GameLog {
    /// Message du log
    pub message: String,
    /// Timestamp du log
    pub timestamp: SystemTime,
}

pub struct Game {
    pub current_map: Option<Map>,
    pub towers: Vec<Tower>,
    pub waves: Option<VecDeque<Wave>>,
    pub current_wave: Option<Wave>,
    pub wave_index: u32,
    pub wave_multiplier: f32,
    pub player_life: i32,
    pub elapsed_time: f32,
    pub spawn_interval: f32, // Intervalle entre les monstres (0 = spawn simultané)
    pub logs: Vec<GameLog>,
    pub log_limit: usize,
    pub money: u32,
}

impl Game {
    pub fn new(towers: Vec<Tower>, player_life: i32, wave_multiplier: f32) -> Self {
        Self {
            current_map: None,
            towers,
            waves: Some(VecDeque::new()),
            current_wave: None,
            wave_index: 0,
            wave_multiplier,
            player_life,
            elapsed_time: 0.0,
            spawn_interval: 1.0,
            logs: Vec::new(),
            log_limit: 100,
            money: 100000000,
        }
    }

    /// Ajoute un nouveau log au jeu
    pub fn add_log(&mut self, message: String) {
        let log = GameLog {
            message: message.clone(),
            timestamp: SystemTime::now(),
        };

        self.logs.push(log);

        // Limiter le nombre de logs
        if self.logs.len() > self.log_limit {
            self.logs.remove(0);
        }

        // Envoyer une notification pour les événements importants
        if message.contains("Game Over") || message.contains("VICTOIRE") {
            Notifier::send_notification("Tower Defense", &message);
        }
    }

    pub fn add_basic_tower(&mut self, position: Position) {
        self.towers.push(BasicTower::positionned(position));
    }

    pub fn add_fire_tower(&mut self, position: Position) {
        self.towers.push(FireTower::positionned(position));
    }

    pub fn add_water_tower(&mut self, position: Position) {}

    pub fn add_earth_tower(&mut self, position: Position) {
        self.towers.push(EarthTower::positionned(position));
    }

    pub fn add_air_tower(&mut self, position: Position) {
        self.towers.push(AirTower::positionned(position));
    }

    pub fn add_sentinel_tower(&mut self, position: Position) {
        self.towers.push(SentinelTower::positionned(position));
    }

    pub fn remove_tower(&mut self, position: Position) {
        if let Some(index) = self
            .towers
            .iter()
            .position(|t| t.position.x == position.x && t.position.y == position.y)
        {
            self.towers.remove(index);
        }
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
        self.add_log(format!(
            "💰 Gain de {} pièces! Total: {}",
            amount, self.money
        ));
    }

    pub fn has_enough_money(&self, amount: u32) -> bool {
        self.money >= amount
    }

    pub fn spend_money(&mut self, amount: u32) -> bool {
        if self.has_enough_money(amount) {
            self.money -= amount;
            self.add_log(format!(
                "💸 Dépense de {} pièces. Reste: {}",
                amount, self.money
            ));
            true
        } else {
            self.add_log(format!("❌ Pas assez de pièces! ({})", self.money));
            false
        }
    }

    pub fn set_spawn_interval(&mut self, interval: f32) {
        self.spawn_interval = interval;
    }

    fn gen_random_wave(&self) -> Wave {
        if let Some(map) = &self.current_map {
            let mut rng = rng();
            let count = rng.random_range(1..=10 + self.wave_index);
            let mut monsters = Vec::new();

            for _ in 0..count as usize {
                let mut monster = map.monsters[rng.random_range(0..map.monsters.len())].clone();
                monster.hp = monster.hp * (1.0 + self.wave_index as f32 * self.wave_multiplier);
                monster.waypoint_idx = 1;

                monsters.push(monster);
            }

            if self.spawn_interval <= 0.0 {
                return Wave::new(Some(monsters));
            } else {
                return Wave::with_staggered_spawn(monsters, self.spawn_interval);
            }
        }

        panic!("No map selected");
    }

    fn start_next_wave(&mut self) {
        if let Some(map) = &self.current_map {
            if self.current_wave.is_none() {
                self.wave_index += 1;

                let wave = self
                    .waves
                    .as_mut()
                    .unwrap()
                    .pop_front()
                    .unwrap_or_else(|| self.gen_random_wave());

                let log_message = format!(
                    "🚩 Démarrage vague {}: {} monstres sur carte '{}'",
                    self.wave_index,
                    wave.monsters.len(),
                    map.name
                );

                self.add_log(log_message);
                self.current_wave = Some(wave);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.current_wave.is_none() {
            self.start_next_wave();
            return;
        }

        let mut logs_to_add = Vec::new();

        let sub_frames = 1;
        let sub_delta = delta_time / sub_frames as f32;
        let start_time = self.elapsed_time.clone();

        let mut towers = Vec::new();
        for i in 0..self.towers.len() {
            if self.towers[i].can_shoot(start_time) {
                towers.push(self.towers[i].clone());
            }
        }

        for i in 0..sub_frames {
            let sub_frame_time = start_time + sub_delta * i as f32;

            for tower in &mut towers {
                if tower.can_shoot(sub_frame_time) {
                    let tower_logs = tower.shoot(self, sub_frame_time);
                    logs_to_add.extend(tower_logs);
                }
            }
        }

        self.towers = towers;
        self.elapsed_time += delta_time;

        if let Some(wave) = &mut self.current_wave {
            let newly_spawned = wave.update_spawns(delta_time);

            for monster in newly_spawned {
                let log_message = format!(
                    "👾 Apparition d'un monstre {}! HP: {:.1}",
                    monster.name, monster.hp
                );
                logs_to_add.push(log_message);
            }

            for monster in wave.monsters.iter_mut() {
                if let Some(map) = &self.current_map
                    && monster.is_alive()
                {
                    monster.advance(map, delta_time);
                }
            }

            // let sub_frames = 1;
            // let sub_delta = delta_time / sub_frames as f32;
            // let start_time = self.elapsed_time;

            // for i in 0..sub_frames {
            //     let sub_frame_time = start_time + sub_delta * i as f32;

            //     for tower in &mut self.towers {
            //         if tower.can_shoot(sub_frame_time) {
            //             let tower_logs = tower.shoot(self, sub_frame_time);
            //             logs_to_add.extend(tower_logs);
            //         }
            //     }
            // }

            // self.elapsed_time += delta_time;

            let mut rem = Vec::new();
            let mut wave_is_empty = false;

            if let Some(map) = &self.current_map {
                for monster in wave.monsters.drain(..) {
                    if monster.is_alive() {
                        if monster.reached_goal(map) {
                            self.player_life -= monster.damage_to_player as i32;
                            let log_message = format!(
                                "⚠️ Monstre {} arrivé, -{} vie(s). Vie joueur: {}",
                                monster.name, monster.damage_to_player, self.player_life
                            );
                            logs_to_add.push(log_message);
                        } else {
                            rem.push(monster);
                        }
                    } else if monster.hp <= 0.0 && monster.active {
                        let reward = 10 + (self.wave_index as u32);
                        logs_to_add.push(format!(
                            "💀 Monstre {} éliminé! +{} pièces",
                            monster.name, reward
                        ));

                        logs_to_add.push(format!(
                            "💰 Gain de {} pièces! Total: {}",
                            reward,
                            self.money + reward
                        ));
                        self.money += reward;
                    } else {
                        rem.push(monster);
                    }
                }
            }

            wave.monsters = rem;
            wave_is_empty = wave.monsters.is_empty();

            for log in logs_to_add {
                self.add_log(log);
            }

            if wave_is_empty {
                let wave_bonus = 20 * self.wave_index as u32;
                self.money += wave_bonus;
                let log_message = format!(
                    "🏆 Vague {} terminée! Bonus de +{} pièces",
                    self.wave_index, wave_bonus
                );
                self.add_log(log_message);
                self.current_wave = None;

                if self.player_life > 0 {
                    let log_message = format!("✅ Préparation de la prochaine vague...");

                    self.add_log(log_message);
                    self.start_next_wave();
                } else {
                    let log_message = "☠️ Game Over! Vous avez perdu!".to_string();
                    self.add_log(log_message);
                }
            }
        }
    }

    pub fn run(&mut self, seconds_per_frame: f32, total_seconds: Option<f32>) {
        let tick = Duration::from_secs_f32(seconds_per_frame);
        let start_time = Instant::now();

        loop {
            let frame_start = Instant::now();

            self.update(seconds_per_frame);

            if self.player_life <= 0 || (self.waves.is_none() && self.current_wave.is_none()) {
                break;
            }

            if let Some(total) = total_seconds {
                if start_time.elapsed().as_secs_f32() >= total {
                    break;
                }
            }

            let elapsed = frame_start.elapsed();
            if elapsed < tick {
                std::thread::sleep(tick - elapsed);
            }
        }
    }

    pub fn upgrade_tower_attack_speed(&mut self, tower_index: usize) -> Result<String, String> {
        if tower_index >= self.towers.len() {
            let message = "❌ Invalid tower index".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        let tower = &self.towers[tower_index];
        let upgrade_type = UpgradeType::AttackSpeed;
        let upgrade_cost = tower.upgrade_cost_for_attribute(upgrade_type);

        // Vérifier si l'amélioration est au maximum (coût = 0)
        if upgrade_cost == 0 {
            let message = "❌ Amélioration déjà au niveau maximum".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        if !self.has_enough_money(upgrade_cost) {
            let message = format!("❌ Missing money ({})", self.money);
            self.add_log(message.clone());
            return Err(message);
        }

        match self.towers[tower_index].upgrade_attack_speed() {
            Ok(_) => {
                self.spend_money(upgrade_cost);
                self.add_log("Attack speed upgraded".to_string());
                Ok("Attack speed upgraded".to_string())
            }
            Err(error) => {
                self.add_log(error.clone());
                Err(error)
            }
        }
    }

    pub fn upgrade_tower_damage(&mut self, tower_index: usize) -> Result<String, String> {
        if tower_index >= self.towers.len() {
            let message = "❌ Invalid tower index".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        let tower = &self.towers[tower_index];
        let upgrade_cost = tower.upgrade_cost_for_attribute(UpgradeType::Damage);

        // Vérifier si l'amélioration est au maximum (coût = 0)
        if upgrade_cost == 0 {
            let message = "❌ Amélioration déjà au niveau maximum".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        if !self.has_enough_money(upgrade_cost) {
            let message = format!("❌ Missing money ({})", self.money);
            self.add_log(message.clone());
            return Err(message);
        }

        match self.towers[tower_index].upgrade_damage() {
            Ok(_) => {
                self.spend_money(upgrade_cost);
                let tower_type = self.towers[tower_index].name.clone();
                self.add_log(format!("🔧 Tour {} améliorée: Dégâts +", tower_type));
                Ok("Damage upgraded".to_string())
            }
            Err(error) => {
                self.add_log(error.clone());
                Err(error)
            }
        }
    }

    pub fn upgrade_tower_range(&mut self, tower_index: usize) -> Result<String, String> {
        if tower_index >= self.towers.len() {
            let message = "❌ Invalid tower index".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        let tower = &self.towers[tower_index];
        let upgrade_type = UpgradeType::Range;
        let upgrade_cost = tower.upgrade_cost_for_attribute(upgrade_type);

        // Vérifier si l'amélioration est au maximum (coût = 0)
        if upgrade_cost == 0 {
            let message = "❌ Amélioration déjà au niveau maximum".to_string();
            self.add_log(message.clone());
            return Err(message);
        }

        if !self.has_enough_money(upgrade_cost) {
            let message = format!("❌ Missing money ({})", self.money);
            self.add_log(message.clone());
            return Err(message);
        }

        match self.towers[tower_index].upgrade_range() {
            Ok(_) => {
                self.spend_money(upgrade_cost);
                self.add_log("Range upgraded".to_string());
                Ok("Range upgraded".to_string())
            }
            Err(error) => {
                self.add_log(error.clone());
                Err(error)
            }
        }
    }
}
