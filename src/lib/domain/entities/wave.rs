use super::monster::Monster;

#[derive(Debug, Clone)]
pub struct Wave {
    pub monsters: Vec<Monster>,
}

impl Wave {
    pub fn new(monsters: Option<Vec<Monster>>) -> Self {
        let active_monsters = if let Some(monsters) = monsters {
            let mut active_monsters = Vec::new();
            for mut monster in monsters {
                monster.spawn_delay = 0.0;
                monster.active = true;
                active_monsters.push(monster);
            }

            active_monsters
        } else {
            vec![]
        };

        Self {
            monsters: active_monsters,
        }
    }

    pub fn with_staggered_spawn(monsters: Vec<Monster>, interval: f32) -> Self {
        let mut result = Vec::new();

        for (i, mut monster) in monsters.into_iter().enumerate() {
            let delay = i as f32 * interval;
            monster.spawn_delay = delay;
            monster.active = delay <= 0.0;
            result.push(monster);
        }

        Self { monsters: result }
    }

    // Met à jour les délais de spawn de tous les monstres
    pub fn update_spawns(&mut self, delta_time: f32) -> Vec<&mut Monster> {
        let mut newly_spawned = Vec::new();

        for monster in &mut self.monsters {
            if monster.update_spawn_status(delta_time) {
                newly_spawned.push(monster);
            }
        }

        newly_spawned
    }
}
