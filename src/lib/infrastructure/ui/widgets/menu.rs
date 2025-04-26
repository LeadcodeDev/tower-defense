use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

pub struct Menu {
    elements: Vec<(String, bool)>,
    selected_index: usize,
}

impl Widget for Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.elements.into_iter().map(|(text, selected)| {
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

        // Ajouter le code pour rendre les éléments du menu au buffer
    }
}
