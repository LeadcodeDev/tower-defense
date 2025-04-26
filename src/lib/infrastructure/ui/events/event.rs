use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

/// Événements que l'application peut recevoir
#[derive(Debug)]
pub enum Event {
    /// Événement de tick pour les mises à jour régulières
    Tick,
    /// Événement de touche du clavier
    Key(KeyEvent),
    /// Événement de souris
    Mouse(MouseEvent),
    /// Événement de redimensionnement du terminal
    Resize(u16, u16),
}

/// Configuration du gestionnaire d'événements
pub struct EventConfig {
    /// Taux de rafraîchissement pour les événements de tick
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(100),
        }
    }
}

/// Gestionnaire d'événements pour l'application
#[derive(Debug)]
pub struct Events {
    /// Receiver pour les événements
    pub receiver: mpsc::Receiver<Event>,
    /// Handle du thread d'événements
    _handle: thread::JoinHandle<()>,
}

impl Events {
    /// Crée un nouveau gestionnaire d'événements avec la configuration fournie
    pub fn new(config: EventConfig) -> Self {
        let (sender, receiver) = mpsc::channel();
        let tick_rate = config.tick_rate;

        // Cloner le sender pour pouvoir l'utiliser dans le thread
        let thread_sender = sender;
        let _handle = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    match event::read().unwrap() {
                        CrosstermEvent::Key(key) => {
                            if thread_sender.send(Event::Key(key)).is_err() {
                                break;
                            }
                        }
                        CrosstermEvent::Mouse(mouse) => {
                            if thread_sender.send(Event::Mouse(mouse)).is_err() {
                                break;
                            }
                        }
                        CrosstermEvent::Resize(width, height) => {
                            if thread_sender.send(Event::Resize(width, height)).is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if thread_sender.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver, _handle }
    }

    /// Récupère le prochain événement
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}
