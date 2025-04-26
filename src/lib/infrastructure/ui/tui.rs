use std::{io, panic};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Terminal};

/// Gestionnaire pour le terminal TUI
pub struct Tui {
    /// Terminal Ratatui
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    /// Crée un nouveau gestionnaire TUI
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    /// Initialise le terminal pour l'utilisation de Ratatui
    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Configurez un hook de panique pour nettoyer le terminal en cas de panique
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            Self::reset()
                .expect("Erreur lors de la réinitialisation du terminal après une panique");
            panic_hook(panic_info);
        }));

        Ok(())
    }

    /// Dessine l'interface utilisateur avec le rendu fourni
    pub fn draw<F>(&mut self, render: F) -> Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(render)?;
        Ok(())
    }

    /// Réinitialise le terminal à son état d'origine
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    /// Réinitialise les paramètres du terminal
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}
