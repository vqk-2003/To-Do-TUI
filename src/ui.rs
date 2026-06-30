use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn render(frame: &mut Frame, app: &mut App) {
    let instruction = Line::from(match app.screen {
        CurrentScreen::Main => vec![
            Span::from("(q) to quit"),
            Span::from(" | "),
            Span::from("(t) to toggle"),
            Span::from(" | "),
            Span::from("(a) to add"),
            Span::from(" | "),
            Span::from("(e) to edit"),
        ],
        CurrentScreen::Editing => vec![Span::from("(Enter) to confirm")],
        CurrentScreen::Deleting => vec![
            Span::from("(y) to confirm"),
            Span::from(" | "),
            Span::from("(n) to cancel"),
        ],
        CurrentScreen::Exiting => vec![
            Span::from("(y) to confirm"),
            Span::from(" | "),
            Span::from("(n) to cancel"),
        ],
    });

    let border = Block::bordered()
        .title(Line::from("To Do App").centered())
        .title_bottom(instruction)
        .border_type(BorderType::Rounded);

    let text = Text::from(
        app.list
            .iter()
            .enumerate()
            .map(|x| {
                if x.0 == app.cursor_line {
                    Line::from(vec![
                        if x.1.0 {
                            Span::from("🗹")
                        } else {
                            Span::from("☐")
                        },
                        Span::from(" | "),
                        Span::from(&x.1.1),
                    ])
                    .style(Style::default().bg(Color::DarkGray))
                } else {
                    Line::from(vec![
                        if x.1.0 {
                            Span::from("🗹")
                        } else {
                            Span::from("☐")
                        },
                        Span::from(" | "),
                        Span::from(&x.1.1),
                    ])
                }
            })
            .collect::<Vec<Line>>(),
    );

    let paragraph = Paragraph::new(text).block(border);

    frame.render_widget(paragraph, frame.area());

    let popup = Rect::new(
        frame.area().x,
        frame.area().height / 3,
        frame.area().width,
        frame.area().height / 3,
    );

    frame.render_widget(Clear, popup);
    frame.render_widget(Block::bordered(), popup);
}
