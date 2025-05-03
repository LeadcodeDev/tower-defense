use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    domain::entities::tower::{TowerKind, TowerUpgradeElementUnit, UpgradeType},
    infrastructure::ui::app::{App, GameAction, TowerType, UiMode, View},
};

/// Gère le rendu de l'interface utilisateur
pub fn render(app: &App, frame: &mut Frame) {
    match app.current_view {
        View::Game => render_game_view(app, frame),
        View::MainMenu => render_main_menu(app, frame),
        View::MapSelection => render_map_selection(app, frame),
        View::Pause => render_pause_menu(app, frame),
        View::GameOver => render_game_over(app, frame),
    }
}

/// Affiche la vue de jeu principale
fn render_game_view(app: &App, frame: &mut Frame) {
    // Diviser l'écran en sections
    let [map_area, info_area] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(frame.area());

    let [header_area, actions_area, logs_area] = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(40),
        Constraint::Percentage(40),
    ])
    .areas(info_area);

    render_header(app, frame, header_area);
    render_map(app, frame, map_area);
    render_actions(app, frame, actions_area);

    // Divisez la zone des logs en deux sections
    let [logs_display_area, monsters_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(logs_area);

    render_monsters_bar(app, frame, monsters_area);
    render_logs(app, frame, logs_display_area);
}

/// Affiche l'en-tête avec des informations de base
fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "Tower Defense",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Vie: {}", app.game.player_life),
            Style::default().fg(Color::Green),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Vague: {}", app.game.wave_index),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("💰 Pièces: {}", app.game.money),
            Style::default().fg(Color::Yellow),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Informations"));

    frame.render_widget(header, area);
}

/// Affiche la carte de jeu
fn render_map(app: &App, frame: &mut Frame, area: Rect) {
    let game = &app.game;

    // Créer une matrice pour stocker les caractères et une autre pour les styles
    let mut map_chars = vec![vec![' '; area.width as usize]; area.height as usize];
    let mut map_styles = vec![vec![Style::default(); area.width as usize]; area.height as usize];

    // Marquer toutes les cellules du chemin avec un style spécial
    for waypoint in &game.map.waypoints {
        if waypoint.x < area.width as i32 && waypoint.y < area.height as i32 {
            map_chars[waypoint.y as usize][waypoint.x as usize] = '.';
            map_styles[waypoint.y as usize][waypoint.x as usize] =
                Style::default().bg(Color::DarkGray).fg(Color::White);
        }
    }

    // Créer un chemin continu entre les waypoints
    if game.map.waypoints.len() > 1 {
        for i in 0..game.map.waypoints.len() - 1 {
            let start = game.map.waypoints[i];
            let end = game.map.waypoints[i + 1];

            // Dessiner une ligne entre les deux waypoints
            let dx = (end.x - start.x).signum();
            let dy = (end.y - start.y).signum();

            let mut x = start.x;
            let mut y = start.y;

            // Commencer par l'horizontal, puis le vertical
            while x != end.x {
                x += dx;
                if x >= 0 && x < area.width as i32 && y >= 0 && y < area.height as i32 {
                    map_chars[y as usize][x as usize] = ' ';
                    map_styles[y as usize][x as usize] =
                        Style::default().bg(Color::DarkGray).fg(Color::White);
                }
            }

            while y != end.y {
                y += dy;
                if x >= 0 && x < area.width as i32 && y >= 0 && y < area.height as i32 {
                    map_chars[y as usize][x as usize] = ' ';
                    map_styles[y as usize][x as usize] =
                        Style::default().bg(Color::DarkGray).fg(Color::White);
                }
            }
        }
    }

    // Dessiner les tourelles
    for (i, tower) in game.towers.iter().enumerate() {
        let pos = tower.position;
        if pos.x < area.width as i32 && pos.y < area.height as i32 {
            let tower_char = match tower.meta.tower_type {
                TowerKind::Basic => 'B',
                TowerKind::Fire => 'F',
                TowerKind::Water => 'W',
                TowerKind::Earth => 'E',
                TowerKind::Air => 'A',
                TowerKind::Lightning => 'L',
                TowerKind::Ice => 'I',
                TowerKind::Poison => 'P',
                TowerKind::Sentinel => 'S',
            };

            map_chars[pos.y as usize][pos.x as usize] = tower_char;

            // Mettre en évidence la tour sélectionnée quand on est en mode sélection sur la carte
            let is_selected = app.tower_selection_on_map
                && app.selected_tower_index.map_or(false, |index| index == i);

            if is_selected {
                map_styles[pos.y as usize][pos.x as usize] = Style::default()
                    .fg(Color::Green)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD);
            } else {
                map_styles[pos.y as usize][pos.x as usize] = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }
        }
    }

    // Dessiner les monstres
    if let Some(wave) = &game.current_wave {
        for monster in &wave.monsters {
            if monster.active {
                let x = monster.position.x;
                let y = monster.position.y;
                if x < area.width as i32 && y < area.height as i32 {
                    map_chars[y as usize][x as usize] = '■';
                    map_styles[y as usize][x as usize] = Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::DarkGray); // Garder le fond du chemin
                }
            }
        }
    }

    // Dessiner le curseur en mode placement ou en mode sélection sur la carte
    if app.ui_mode == UiMode::Placement
        || (app.ui_mode == UiMode::TowerSelection && app.tower_selection_on_map)
    {
        let cursor_x = app.cursor_position.x;
        let cursor_y = app.cursor_position.y;
        if cursor_x < area.width as i32 && cursor_y < area.height as i32 {
            // En mode placement normal, afficher X, en mode sélection sur carte afficher un symbole différent
            let is_upgrade_mode = app.selected_index < app.available_actions.len()
                && app.selected_tower.is_none()
                && app.available_actions[app.selected_index] == GameAction::UpgradeTower;

            if is_upgrade_mode {
                map_styles[cursor_y as usize][cursor_x as usize] = Style::default()
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            } else {
                map_chars[cursor_y as usize][cursor_x as usize] = '×';
                map_styles[cursor_y as usize][cursor_x as usize] =
                    Style::default().add_modifier(Modifier::BOLD);
            }
        }
    }

    // Convertir la grille en texte stylisé pour l'affichage
    let map_text: Vec<Line> = (0..map_chars.len())
        .map(|y| {
            let spans: Vec<Span> = (0..map_chars[y].len())
                .map(|x| Span::styled(map_chars[y][x].to_string(), map_styles[y][x]))
                .collect();
            Line::from(spans)
        })
        .collect();

    let map_widget = Paragraph::new(map_text)
        .block(Block::default().borders(Borders::ALL).title("Carte"))
        .style(Style::default());

    frame.render_widget(map_widget, area);
}

/// Affiche la barre d'informations
fn render_monsters_bar(app: &App, frame: &mut Frame, area: Rect) {
    // Diviser en sections d'information
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Monstres actifs
            Constraint::Percentage(50), // Instructions
        ])
        .split(area);

    // Section des monstres actifs
    let mut monster_items = Vec::new();
    if let Some(wave) = &app.game.current_wave {
        for (i, monster) in wave.monsters.iter().enumerate().filter(|(_, m)| m.active) {
            monster_items.push(ListItem::new(format!(
                "{}: HP {:.1}",
                monster.name, monster.hp
            )));

            // Limiter le nombre de monstres affichés
            if i >= 4 {
                monster_items.push(ListItem::new("..."));
                break;
            }
        }
    }

    let monsters_list = List::new(monster_items)
        .block(Block::default().borders(Borders::ALL).title("Monstres"))
        .style(Style::default().fg(Color::White));

    // Section des instructions
    let mut instructions_text = vec![Line::from("q: Quitter | p: Pause")];

    match app.ui_mode {
        UiMode::Normal => {
            instructions_text.push(Line::from("↑ ↓: Sélectionner action"));
            instructions_text.push(Line::from("Enter: Confirmer action"));
        }
        UiMode::TowerSelection => {
            if app.tower_selection_on_map {
                instructions_text.push(Line::from("↑ ↓ ← →: Naviguer entre les tours"));
                instructions_text.push(Line::from("Enter: Sélectionner | Esc: Annuler"));
            } else {
                instructions_text.push(Line::from("↑ ↓: Sélectionner type"));
                instructions_text.push(Line::from("Enter: Choisir | Esc: Annuler"));
            }
        }
        UiMode::Placement => {
            // Déterminons si nous sommes en mode amélioration
            let is_upgrade_mode = app.selected_index < app.available_actions.len()
                && app.selected_tower.is_none()
                && app.available_actions[app.selected_index] == GameAction::UpgradeTower;

            if is_upgrade_mode {
                instructions_text.push(Line::from("↑ ↓ ← →: Déplacer le curseur sur la carte"));
                instructions_text.push(Line::from("Enter: Sélectionner la tour | Esc: Annuler"));
            } else {
                instructions_text.push(Line::from("↑ ↓ ← →: Déplacer curseur"));
                instructions_text.push(Line::from("Enter: Confirmer | Esc: Annuler"));
            }
        }
        UiMode::TowerUpgrade => {
            instructions_text.push(Line::from("↑ ↓: Sélectionner amélioration"));
            instructions_text.push(Line::from("Enter: Améliorer | Esc: Annuler"));
        }
    }

    let instructions = Paragraph::new(instructions_text)
        .block(Block::default().borders(Borders::ALL).title("Commandes"))
        .style(Style::default().fg(Color::White));

    frame.render_widget(monsters_list, chunks[0]);
    frame.render_widget(instructions, chunks[1]);
}

fn render_actions(app: &App, frame: &mut Frame, area: Rect) {
    let mut action_items = Vec::new();

    match app.ui_mode {
        UiMode::Normal => {
            // Afficher les actions principales
            for (i, action) in app.available_actions.iter().enumerate() {
                let text = match action {
                    GameAction::BuildTower => "🧱 Construire une tour",
                    GameAction::RemoveTower => "🗑️ Supprimer une tour",
                    GameAction::UpgradeTower => "🔧 Améliorer une tour existante",
                };

                // Mettre en surbrillance l'action sélectionnée
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                action_items.push(ListItem::new(Span::styled(text, style)));
            }

            let actions_list = List::new(action_items)
                .block(Block::default().borders(Borders::ALL).title("Actions"))
                .style(Style::default().fg(Color::White));

            frame.render_widget(actions_list, area);
        }
        UiMode::TowerSelection => {
            // Afficher les types de tours disponibles
            for (i, tower_type) in app.available_towers.iter().enumerate() {
                let text = match tower_type {
                    TowerType::Basic => format!(
                        "Tour basique (B) - Attaque basique - 💰 {}",
                        tower_type.cost()
                    ),
                    TowerType::Fire => format!(
                        "Tour de feu (F) - Attaque en zone - 💰 {}",
                        tower_type.cost()
                    ),
                    TowerType::Water => {
                        format!("Tour d'eau (W) - Ralentissement - 💰 {}", tower_type.cost())
                    }
                    TowerType::Earth => {
                        format!("Tour de terre (E) - Résistance - 💰 {}", tower_type.cost())
                    }
                    TowerType::Air => {
                        format!("Tour d'air (A) - Attaque rapide - 💰 {}", tower_type.cost())
                    }
                    TowerType::Lightning => {
                        format!(
                            "Tour de foudre (L) - Étourdissement - 💰 {}",
                            tower_type.cost()
                        )
                    }
                    TowerType::Ice => {
                        format!("Tour de glace (I) - Gel - 💰 {}", tower_type.cost())
                    }
                    TowerType::Poison => {
                        format!(
                            "Tour de poison (P) - Dégâts continus - 💰 {}",
                            tower_type.cost()
                        )
                    }
                    TowerType::Sentinel => {
                        format!(
                            "Tour de sentinelle (S) - Détection - 💰 {}",
                            tower_type.cost()
                        )
                    }
                };

                // Mettre en surbrillance la tour sélectionnée
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                action_items.push(ListItem::new(Span::styled(text, style)));
            }

            let tower_list = List::new(action_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Types de tours"),
                )
                .style(Style::default().fg(Color::White));

            frame.render_widget(tower_list, area);
        }
        UiMode::Placement => {
            // En mode placement, afficher des instructions
            let (mode_text, instructions) = if let Some(tower_type) = app.selected_tower {
                // Mode placement de tour
                let tower_name = match tower_type {
                    TowerType::Basic => "basique",
                    TowerType::Fire => "de feu",
                    TowerType::Water => "d'eau",
                    TowerType::Earth => "de terre",
                    TowerType::Air => "d'air",
                    TowerType::Lightning => "de foudre",
                    TowerType::Ice => "de glace",
                    TowerType::Poison => "de poison",
                    TowerType::Sentinel => "de sentinelle",
                };

                (
                    format!("Mode placement - Tour {}", tower_name),
                    vec![
                        Line::from(""),
                        Line::from("Utilisez les flèches pour positionner le curseur"),
                        Line::from("Appuyez sur Enter pour placer la tour"),
                        Line::from("Appuyez sur Escape pour annuler"),
                    ],
                )
            } else {
                // Déterminons si nous sommes en mode suppression ou amélioration
                let is_upgrade_mode = app.selected_index < app.available_actions.len()
                    && app.selected_tower.is_none()
                    && app.available_actions[app.selected_index] == GameAction::UpgradeTower;

                if is_upgrade_mode {
                    // Mode sélection de tour pour amélioration
                    (
                        "Mode amélioration de tour".to_string(),
                        vec![
                            Line::from(""),
                            Line::from(
                                "Utilisez les flèches pour déplacer le curseur sur la carte",
                            ),
                            Line::from(
                                "Positionnez-vous sur une tour existante (lettres B, F, W, E, A)",
                            ),
                            Line::from("Appuyez sur Enter pour sélectionner la tour à améliorer"),
                            Line::from("Appuyez sur Escape pour annuler"),
                        ],
                    )
                } else {
                    // Mode suppression de tour
                    (
                        "Mode suppression de tour".to_string(),
                        vec![
                            Line::from(""),
                            Line::from("Utilisez les flèches pour positionner le curseur"),
                            Line::from("Appuyez sur Enter pour supprimer la tour"),
                            Line::from("Appuyez sur Escape pour annuler"),
                        ],
                    )
                }
            };

            let mut placement_lines = vec![Line::from(Span::styled(
                mode_text,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ))];

            placement_lines.extend(instructions);

            let placement_info = Paragraph::new(placement_lines)
                .block(Block::default().borders(Borders::ALL).title("Placement"))
                .style(Style::default().fg(Color::White));

            frame.render_widget(placement_info, area);
        }
        UiMode::TowerUpgrade => {
            // Vérifier si nous avons un menu d'amélioration
            if let Some(upgrade_menu) = &app.upgrade_menu {
                let tower_index = upgrade_menu.tower_index;

                if tower_index < app.game.towers.len() {
                    let tower = &app.game.towers[tower_index];
                    let position = tower.position;
                    let level = tower.level;

                    // Afficher d'abord les informations sur la tour
                    let tower_info = format!(
                        "Tour {} (x:{}, y:{}) - Level {}",
                        tower.name.clone(),
                        position.x,
                        position.y,
                        level
                    );

                    action_items.push(ListItem::new(Span::styled(
                        tower_info,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));

                    action_items.push(ListItem::new(""));
                    action_items.push(ListItem::new("Choisissez une amélioration:"));
                    action_items.push(ListItem::new(""));

                    // Afficher les options d'amélioration
                    for (i, (upgrade_type, description)) in
                        upgrade_menu.available_upgrades.iter().enumerate()
                    {
                        let cost = tower.upgrade_cost_for_attribute(*upgrade_type);
                        let is_maxed = cost == 0;

                        // Récupérer les valeurs actuelles
                        let current_value = match upgrade_type {
                            UpgradeType::AttackSpeed => {
                                if let Some(attack_speed) = &tower.stats.attacks_per_second {
                                    attack_speed.base
                                } else {
                                    0.0
                                }
                            }
                            UpgradeType::Damage => {
                                if let Some(damage) = &tower.stats.damage {
                                    damage.base
                                } else {
                                    0.0
                                }
                            }
                            UpgradeType::Range => tower.stats.range.base,
                        };

                        let bonus = if is_maxed {
                            // Format pour les améliorations au maximum - sans MAX à la fin
                            match upgrade_type {
                                UpgradeType::AttackSpeed => {
                                    format!("⚡️ {:.2}/s Attack speed (MAX)", current_value)
                                }
                                UpgradeType::Damage => {
                                    format!("💥 {:.2} Damage (MAX)", current_value)
                                }
                                UpgradeType::Range => {
                                    format!("🔍 {:.2} Range (MAX)", current_value)
                                }
                            }
                        } else {
                            // Format pour les améliorations normales
                            let modifier = match upgrade_type {
                                UpgradeType::AttackSpeed => {
                                    match tower
                                        .upgrades
                                        .attacks_speed
                                        .as_ref()
                                        .unwrap()
                                        .value_multiplier_unit
                                    {
                                        TowerUpgradeElementUnit::Percent => format!(
                                            "x{:.2}%",
                                            tower
                                                .upgrades
                                                .attacks_speed
                                                .as_ref()
                                                .unwrap()
                                                .value_multiplier
                                        ),
                                        TowerUpgradeElementUnit::Unit => format!(
                                            "+{}",
                                            tower
                                                .upgrades
                                                .attacks_speed
                                                .as_ref()
                                                .unwrap()
                                                .value_multiplier
                                        ),
                                    }
                                }
                                UpgradeType::Damage => {
                                    match tower
                                        .upgrades
                                        .damage
                                        .as_ref()
                                        .unwrap()
                                        .value_multiplier_unit
                                    {
                                        TowerUpgradeElementUnit::Percent => {
                                            format!(
                                                "x{:.2}%",
                                                tower
                                                    .upgrades
                                                    .damage
                                                    .as_ref()
                                                    .unwrap()
                                                    .value_multiplier
                                            )
                                        }
                                        TowerUpgradeElementUnit::Unit => {
                                            format!(
                                                "+{}",
                                                tower
                                                    .upgrades
                                                    .damage
                                                    .as_ref()
                                                    .unwrap()
                                                    .value_multiplier
                                            )
                                        }
                                    }
                                }
                                UpgradeType::Range => {
                                    match tower
                                        .upgrades
                                        .range
                                        .as_ref()
                                        .unwrap()
                                        .value_multiplier_unit
                                    {
                                        TowerUpgradeElementUnit::Percent => {
                                            format!(
                                                "x{:.2}%",
                                                tower
                                                    .upgrades
                                                    .range
                                                    .as_ref()
                                                    .unwrap()
                                                    .value_multiplier
                                            )
                                        }
                                        TowerUpgradeElementUnit::Unit => {
                                            format!(
                                                "+{}",
                                                tower
                                                    .upgrades
                                                    .range
                                                    .as_ref()
                                                    .unwrap()
                                                    .value_multiplier
                                            )
                                        }
                                    }
                                }
                            };

                            // Format pour les améliorations normales avec le coût
                            format!("{} ({}), 💰{:.2}", description, modifier, cost)
                        };

                        // Mettre en surbrillance l'option sélectionnée
                        let style = if i == upgrade_menu.selected_upgrade {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else if is_maxed {
                            // Style spécial pour les améliorations au max
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };

                        action_items.push(ListItem::new(Span::styled(bonus, style)));
                    }
                }
            } else {
                // Afficher la liste des tours disponibles pour amélioration
                action_items.push(ListItem::new(Span::styled(
                    "Sélectionnez une tour à améliorer:",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                action_items.push(ListItem::new(""));

                // Afficher toutes les tours disponibles
                for (i, tower) in app.game.towers.iter().enumerate() {
                    let position = tower.position;
                    let level = tower.level;
                    let cost = tower.upgrade_cost(level);

                    let tower_info = format!(
                        "Tour {} ({},{}) - Niveau {} - 💰 {} pour améliorer",
                        tower.name.clone(),
                        position.x,
                        position.y,
                        level,
                        cost
                    );

                    // Mettre en surbrillance la tour sélectionnée
                    let style = if i == app.selected_index {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    action_items.push(ListItem::new(Span::styled(tower_info, style)));
                }

                // Si aucune tour n'est disponible
                if app.game.towers.is_empty() {
                    action_items.push(ListItem::new("Aucune tour disponible pour amélioration"));
                }
            }

            let upgrade_list = List::new(action_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Améliorer une tour"),
                )
                .style(Style::default().fg(Color::White));

            frame.render_widget(upgrade_list, area);
        }
    }
}

/// Affiche les logs du jeu
fn render_logs(app: &App, frame: &mut Frame, area: Rect) {
    let logs = &app.game.logs;

    // Créer une liste des logs à afficher (du plus récent au plus ancien)
    let log_items: Vec<ListItem> = logs
        .iter()
        .rev()
        .map(|log| {
            ListItem::new(Line::from(vec![Span::styled(
                log.message.clone(),
                Style::default().fg(Color::White),
            )]))
        })
        .collect();

    let logs_list = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title("Logs"))
        .style(Style::default().fg(Color::White));

    frame.render_widget(logs_list, area);
}

/// Affiche le menu principal
fn render_main_menu(app: &App, frame: &mut Frame) {
    // Créer un titre et des options pour le menu principal
    let title = Line::from(Span::styled(
        "TOWER DEFENSE",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ));

    let items = vec![
        ("Démarrer une nouvelle partie", app.selected_index == 0),
        ("Quitter", app.selected_index == 1),
    ];

    let menu_items: Vec<Line> = items
        .into_iter()
        .map(|(text, selected)| {
            if selected {
                Line::from(Span::styled(
                    format!("> {}", text),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!("  {}", text),
                    Style::default().fg(Color::White),
                ))
            }
        })
        .collect();

    let mut all_lines = vec![
        title,
        Line::from(""),
        Line::from(Span::styled(
            "Défendez votre territoire contre des vagues d'ennemis",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];
    all_lines.extend(menu_items);

    // Ajouter des instructions
    all_lines.push(Line::from(""));
    all_lines.push(Line::from(Span::styled(
        "Utilisez ↑↓ pour naviguer et Entrée pour sélectionner",
        Style::default().fg(Color::Gray),
    )));

    let menu = Paragraph::new(all_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu Principal"),
        )
        .alignment(ratatui::layout::Alignment::Center);

    // Centrer le menu sur l'écran
    frame.render_widget(menu, centered_rect(60, 40, frame.area()));
}

/// Affiche le menu de pause
fn render_pause_menu(app: &App, frame: &mut Frame) {
    // Afficher le jeu en arrière-plan
    render_game_view(app, frame);

    let items = vec![
        ("Reprendre", app.selected_index == 0),
        ("Quitter", app.selected_index == 1),
    ];

    let menu_items: Vec<Line> = items
        .into_iter()
        .map(|(text, selected)| {
            if selected {
                Line::from(Span::styled(
                    format!("> {}", text),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!("  {}", text),
                    Style::default().fg(Color::White),
                ))
            }
        })
        .collect();

    let title = Line::from(Span::styled(
        "PAUSE",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ));

    let mut all_lines = vec![title, Line::from("")];
    all_lines.extend(menu_items);

    let menu = Paragraph::new(all_lines)
        .block(Block::default().borders(Borders::ALL).title("Pause"))
        .style(Style::default().fg(Color::White));

    frame.render_widget(menu, centered_rect(30, 20, frame.area()));
}

/// Affiche l'écran de fin de jeu
fn render_game_over(app: &App, frame: &mut Frame) {
    let title = if app.game.player_life <= 0 {
        "GAME OVER"
    } else {
        "VICTOIRE!"
    };

    let items = vec![
        ("Nouvelle partie", app.selected_index == 0),
        ("Quitter", app.selected_index == 1),
    ];

    let menu_items: Vec<Line> = items
        .into_iter()
        .map(|(text, selected)| {
            if selected {
                Line::from(Span::styled(
                    format!("> {}", text),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!("  {}", text),
                    Style::default().fg(Color::White),
                ))
            }
        })
        .collect();

    let title = Line::from(Span::styled(
        title,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ));

    let mut all_lines = vec![
        title,
        Line::from(""),
        Line::from(format!("Vagues complétées: {}", app.game.wave_index)),
        Line::from(""),
    ];
    all_lines.extend(menu_items);

    let menu = Paragraph::new(all_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Fin de partie"),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(menu, centered_rect(30, 20, frame.area()));
}

/// Helper pour créer un rectangle centré
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Affiche l'écran de sélection de carte
fn render_map_selection(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Créer un titre pour l'écran
    let title = Paragraph::new(Line::from(Span::styled(
        "SÉLECTION DE CARTE",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Tower Defense"),
    )
    .alignment(ratatui::layout::Alignment::Center);

    // Diviser l'écran en sections
    let chunks = Layout::vertical([
        Constraint::Length(3), // Pour le titre
        Constraint::Min(10),   // Pour la liste des cartes
        Constraint::Length(3), // Pour les instructions
    ])
    .split(area);

    frame.render_widget(title, chunks[0]);

    // Créer la liste des cartes
    let mut map_items = Vec::new();

    for (idx, map_type) in app.available_maps.iter().enumerate() {
        let is_selected = idx == app.selected_index;
        let map_name = map_type.get_name();
        let map_desc = map_type.get_description();

        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let mut content = Vec::new();

        // Ajouter une flèche pour indiquer la sélection
        if is_selected {
            content.push(Span::styled("> ", style));
        } else {
            content.push(Span::styled("  ", style));
        }

        // Ajouter le nom de la carte
        content.push(Span::styled(map_name, style));
        content.push(Span::raw(" - "));
        content.push(Span::styled(map_desc, Style::default().fg(Color::Gray)));

        map_items.push(ListItem::new(Line::from(content)));
    }

    let maps_list = List::new(map_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cartes disponibles"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_widget(maps_list, chunks[1]);

    // Ajouter des instructions
    let instructions = Paragraph::new(Line::from(vec![
        Span::styled("Utilisez ", Style::default().fg(Color::Gray)),
        Span::styled(
            "↑↓",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" pour naviguer et ", Style::default().fg(Color::Gray)),
        Span::styled(
            "Entrée",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" pour sélectionner.", Style::default().fg(Color::Gray)),
    ]))
    .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(instructions, chunks[2]);
}
