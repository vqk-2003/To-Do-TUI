use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen};

pub fn render(frame: &mut Frame, app: &mut App) {
    if let CurrentScreen::Main
    | CurrentScreen::Editing
    | CurrentScreen::Adding
    | CurrentScreen::Deleting = app.screen
    {
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

        let paragraph = Paragraph::new(text).block(border).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, frame.area());
    }

    if let CurrentScreen::Editing | CurrentScreen::Adding = app.screen {
        let popup_area = frame
            .area()
            .centered(Constraint::Percentage(80), Constraint::Percentage(30));

        frame.render_widget(Clear, popup_area);

        let border = Block::bordered()
            .title_top(
                Line::from(match app.screen {
                    CurrentScreen::Editing => "Editing",
                    CurrentScreen::Adding => "Adding",
                    _ => unreachable!(),
                })
                .centered(),
            )
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(Text::from(Line::from(vec![
            Span::from(app.editing_text.clone()),
            Span::from("█"),
        ])))
        .wrap(Wrap { trim: true })
        .block(border);

        frame.render_widget(paragraph, popup_area);

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

    if let CurrentScreen::Deleting = app.screen {
        let popup_area = frame
            .area()
            .centered(Constraint::Length(40), Constraint::Length(3));

        frame.render_widget(Clear, popup_area);

        let border = Block::bordered()
            .title_top(Line::from("Deleting").centered())
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(Text::from(
            Line::from("Do you want to delete this entry?").centered(),
        ))
        .block(border);

        frame.render_widget(paragraph, popup_area);

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
        let popup_area = frame
            .area()
            .centered(Constraint::Length(50), Constraint::Length(3));

        frame.render_widget(Clear, popup_area);

        let border = Block::bordered()
            .title_top(Line::from("Exiting").centered())
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(Text::from(
            Line::from("Do you want to save this to file? (y/n)").centered(),
        ))
        .block(border);

        frame.render_widget(paragraph, popup_area);

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
