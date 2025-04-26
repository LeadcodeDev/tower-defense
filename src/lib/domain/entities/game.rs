use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use rand::{Rng, rng};

use super::{
    map::Map,
    monster::Monster,
    tower::{Tower, TowerType},
    wave::Wave,
};

pub struct Game {
    pub maps: Vec<Map>,
    pub current_map: usize,
    pub towers: Vec<TowerType>,
    pub waves: VecDeque<Wave>,
    pub current_wave: Option<Wave>,
    pub wave_index: u32,
    pub wave_multiplier: f32,
    pub player_life: i32,
    pub base_prototypes: Vec<Monster>,
    pub elapsed_time: f32,
    pub spawn_interval: f32, // Intervalle entre les monstres (0 = spawn simultané)
}

impl Game {
    pub fn new(
        maps: Vec<Map>,
        waves: Vec<Wave>,
        towers: Vec<TowerType>,
        player_life: i32,
        wave_multiplier: f32,
    ) -> Self {
        let bp = waves
            .first()
            .map(|w| w.monsters.clone())
            .unwrap_or_default();

        Self {
            maps,
            current_map: 0,
            towers,
            waves: VecDeque::from(waves),
            current_wave: None,
            wave_index: 0,
            wave_multiplier,
            player_life,
            base_prototypes: bp,
            elapsed_time: 0.0,
            spawn_interval: 1.0, // Par défaut, 1 seconde d'intervalle
        }
    }

    // Permet de configurer l'intervalle de spawn (0 = spawn simultané)
    pub fn set_spawn_interval(&mut self, interval: f32) {
        self.spawn_interval = interval;
    }

    fn gen_random_wave(&self) -> Wave {
        let mut rng = rng();
        let base = self.base_prototypes.len().max(1) as u32;
        let count = rng.random_range(1..=base + self.wave_index);
        let mut monsters = Vec::new();

        for i in 0..count as usize {
            let p = &self.base_prototypes[i % self.base_prototypes.len()];
            monsters.push(Monster {
                name: p.name.clone(),
                hp: p.hp * (1.0 + self.wave_index as f32 * self.wave_multiplier),
                position: self.maps[self.current_map].waypoints[0],
                movement_speed: p.movement_speed,
                waypoint_idx: 1,
                resistances: p.resistances.clone(),
                damage_to_player: p.damage_to_player,
                distance_moved: 0.0,
                spawn_delay: 0.0, // Sera configuré ci-dessous
                active: false,    // Sera configuré ci-dessous
            });
        }

        // Option de configuration: choisir si on veut un spawn séquentiel ou simultané
        if self.spawn_interval <= 0.0 {
            // Spawn simultané
            return Wave::new(monsters);
        } else {
            // Spawn séquentiel avec intervalle
            return Wave::with_staggered_spawn(monsters, self.spawn_interval);
        }
    }

    fn start_next_wave(&mut self) {
        if self.current_wave.is_none() {
            self.wave_index += 1;
            let w = self
                .waves
                .pop_front()
                .unwrap_or_else(|| self.gen_random_wave());
            println!(
                "🚩 Démarrage vague {}: {} monstres sur carte '{}'",
                self.wave_index,
                w.monsters.len(),
                self.maps[self.current_map].name
            );
            self.current_wave = Some(w);
        }
    }
    fn update(&mut self, delta_time: f32) {
        // Gérer le démarrage d'une nouvelle vague si nécessaire
        if self.current_wave.is_none() {
            self.start_next_wave();
            return; // Attendre le prochain tick pour commencer à traiter la vague
        }

        // À ce stade, on est sûr d'avoir une vague
        let map = &self.maps[self.current_map];

        if let Some(wave) = &mut self.current_wave {
            // Mettre à jour les délais de spawn et obtenir les monstres nouvellement activés
            let newly_spawned = wave.update_spawns(delta_time);

            // Afficher un message pour les monstres qui viennent d'apparaître
            for monster in newly_spawned {
                println!("👾 Apparition d'un monstre! HP: {:.1}", monster.hp);
            }

            // Déplacement des monstres avec delta_time pour vitesse en cases/seconde
            for m in wave.monsters.iter_mut() {
                if m.is_alive() {
                    m.advance(map, delta_time);
                }
            }

            // Tirs des tourelles avec simulation de sous-frames pour augmenter la précision
            // Diviser le delta_time en sous-frames pour avoir plus de tirs
            let sub_frames = 10; // Augmenté à 10 sous-frames pour plus de précision
            let sub_delta = delta_time / sub_frames as f32;

            let start_time = self.elapsed_time;

            for i in 0..sub_frames {
                // Calculer le temps pour ce sous-frame
                let sub_frame_time = start_time + sub_delta * i as f32;

                for tower in &mut self.towers {
                    if tower.can_shoot(sub_frame_time) {
                        tower.shoot(wave, sub_frame_time);
                    }
                }
            }

            // Avancer le temps global
            self.elapsed_time += delta_time;

            // Gestion des monstres arrivés/morts
            let mut rem = Vec::new();
            for m in wave.monsters.drain(..) {
                if m.is_alive() {
                    if m.reached_goal(map) {
                        self.player_life -= m.damage_to_player as i32;
                        println!(
                            "⚠️ Monstre arrivé, -{} vie(s). Vie joueur: {}",
                            m.damage_to_player, self.player_life
                        );
                    } else {
                        rem.push(m);
                    }
                } else if m.hp <= 0.0 && m.active {
                    // Monstre tué (et pas juste inactif à cause du délai)
                    println!("💀 Monstre {} éliminé!", m.name);
                } else {
                    // Monstre inactif ou en attente de spawn
                    rem.push(m);
                }
            }
            wave.monsters = rem;

            // Vérifier si la vague est terminée (aucun monstre vivant ou en attente)
            let has_pending = wave.monsters.iter().any(|m| !m.active);
            let has_alive = wave.monsters.iter().any(|m| m.is_alive());

            if !has_pending && !has_alive {
                println!(
                    "✅ Vague {} terminée sur carte '{}'",
                    self.wave_index, map.name
                );
                self.current_wave = None;
            }
        }
    }

    pub fn run(&mut self, seconds_per_frame: f32, total_seconds: Option<f32>) {
        let tick = Duration::from_secs_f32(seconds_per_frame);
        let start_time = Instant::now();

        loop {
            let frame_start = Instant::now();

            // Mettre à jour avec le delta time
            self.update(seconds_per_frame);

            // Vérifier si la partie est terminée
            if self.player_life <= 0 || (self.waves.is_empty() && self.current_wave.is_none()) {
                break;
            }

            if let Some(total) = total_seconds {
                if start_time.elapsed().as_secs_f32() >= total {
                    break;
                }
            }

            // Respecter le délai par frame
            let elapsed = frame_start.elapsed();
            if elapsed < tick {
                std::thread::sleep(tick - elapsed);
            }
        }

        println!(
            "{}",
            if self.player_life <= 0 {
                "💀 Game Over"
            } else {
                "🎉 Victoire !"
            }
        );
    }
}
