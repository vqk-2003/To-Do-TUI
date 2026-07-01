use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::{App, CurrentScreen};

//
pub fn render(frame: &mut Frame, app: &mut App) {
    if let CurrentScreen::Main = app.screen {
        let instruction = Line::from(vec![
            Span::from("(Enter) to toggle"),
            Span::from(" | "),
            Span::from("(a) to add"),
            Span::from(" | "),
            Span::from("(e) to edit"),
            Span::from(" | "),
            Span::from("(d) to edit"),
            Span::from(" | "),
            Span::from("(q) to quit"),
        ]);

        let border = Block::bordered()
            .title_top(Line::from("To Do App").centered())
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
    }

    if let CurrentScreen::Editing | CurrentScreen::Adding = app.screen {
        let instruction = Line::from(vec![
            Span::from("(Enter) to confirm"),
            Span::from(" | "),
            Span::from("(ESC) to cancel"),
        ]);

        let border = Block::bordered()
            .title_top(Line::from("To Do App").centered())
            .title_bottom(instruction)
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(Text::from(Line::from(vec![
            Span::from(app.editing_text.clone()),
            Span::from("█"),
        ])))
        .block(border);

        frame.render_widget(paragraph, frame.area());
    }

    if let CurrentScreen::Deleting = app.screen {
        let instruction = Line::from(vec![
            Span::from("(Enter) to confirm"),
            Span::from(" | "),
            Span::from("(ESC) to cancel"),
        ]);

        let border = Block::bordered()
            .title_top(Line::from("To Do App").centered())
            .title_bottom(instruction)
            .border_type(BorderType::Rounded);

        frame.render_widget(border, frame.area());
    }

    if let CurrentScreen::Exiting = app.screen {
        let instruction = Line::from(vec![
            Span::from("(y) to save"),
            Span::from(" | "),
            Span::from("(n) to skip save"),
            Span::from(" | "),
            Span::from("(ESC) to cancel"),
        ]);

        let border = Block::bordered()
            .title_top(Line::from("To Do App").centered())
            .title_bottom(instruction)
            .border_type(BorderType::Rounded);

        frame.render_widget(border, frame.area());
    }
}
