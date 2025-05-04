use rust_tower::{
    domain::entities::{game::Game, wave::Wave},
    infrastructure::ui::{
        app::{App, View},
        tui::Tui,
    },
};

fn main() -> color_eyre::Result<()> {
    // Initialiser color-eyre pour une gestion d'erreur améliorée
    color_eyre::install()?;

    // Créer le jeu sous-jacent
    let game = create_game();

    // Créer notre application TUI
    let mut app = App::new(game);
    app.current_view = View::MainMenu; // Commencer par le menu principal

    // Initialiser TUI
    let mut tui = Tui::new()?;
    tui.init()?;

    // Configurer le gestionnaire d'événements
    app.run(&mut tui)?;

    tui.exit()?;
    Ok(())
}

fn create_game() -> Game {
    Game::new(vec![], 10, 1.0)
}
