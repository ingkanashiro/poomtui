use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{self, Event},
        terminal,
    },
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType::Thick, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    tech_scores: Vec<Score>,
    pres_scores: Vec<Score>,

    deductions: Vec<Deduction>,
}

#[derive(Debug, Default)]
struct Deduction {
    value: f32,
    desc: String,
}

#[derive(Debug, Default)]
struct Score {
    value: f32,
}

fn main() -> Result<()> {
    let mut state = AppState::default();

    // TEST VALUES FOR DEDUCTIONS LIST
    state.deductions.push(Deduction{
        value: 0.3,
        desc: String::from("missed mandatory dwit kubi")
    });

    state.deductions.push(Deduction{
        value: 0.3,
        desc: String::from("missed mandatory boom seogi")
    });
    state.deductions.push(Deduction{
        value: 0.1,
        desc: String::from("overtime")
    });


    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| render(f, app_state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(Thick)
        .fg(Color::Red)
        .render(border_area, frame.buffer_mut());

    List::new(
        app_state
            .deductions
            .iter()
            .map(|x| ListItem::from(format!("* [{}] {}", x.value.clone(), x.desc.clone()))),
    )
    .render(inner_area, frame.buffer_mut());
}
