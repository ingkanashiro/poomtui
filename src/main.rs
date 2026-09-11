use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame, crossterm::{
        event::{self, Event, KeyEvent}, terminal,
    }, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, widgets::{Block, BorderType::{Double, Thick}, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    tech_scores: Vec<Score>,
    pres_scores: Vec<Score>,

    deductions: Vec<Deduction>,
    deductions_state: ListState,

    is_add_deduction: bool,
    is_add_deduction_hard: bool,
    input_value: String,
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

enum FormAction {
    None,
    Submit,
    Escape
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_deduction = false;

    // TEST VALUES FOR DEDUCTIONS LIST
    state.deductions.push(Deduction {
        value: 0.3,
        desc: String::from("missed mandatory dwit kubi"),
    });

    state.deductions.push(Deduction {
        value: 0.3,
        desc: String::from("missed mandatory boom seogi"),
    });
    state.deductions.push(Deduction {
        value: 0.1,
        desc: String::from("overtime"),
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
            
            if app_state.is_add_deduction {
                match handle_add_deduction(key, app_state) {
                    FormAction::None => {},
                    FormAction::Submit => {
                        app_state.is_add_deduction = false;

                        if app_state.is_add_deduction_hard {
                            app_state.deductions.push(Deduction {
                                value: 0.1,
                                desc: app_state.input_value.clone(),
                            })
                        }
                        else {
                            app_state.deductions.push(Deduction {
                                value: 0.3,
                                desc: app_state.input_value.clone(),
                            })
                        }

                        app_state.input_value.clear();
                    },
                    FormAction::Escape => {
                        app_state.is_add_deduction = false;
                        app_state.input_value.clear();
                    },
                }
            } 
            else {    
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn handle_add_deduction(key: KeyEvent, app_state: &mut AppState) -> FormAction {

    match key.code {

        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }

        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }

        event::KeyCode::Enter => {
            return FormAction::Submit;
        }

        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        _ => {}
    }
    
    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'ñ' => {
                app_state.deductions_state.select_previous();
            }
            '{' => {
                app_state.deductions_state.select_next();
            }
            '+' => {
                app_state.deductions_state.select_last();
                app_state.is_add_deduction = true;
                app_state.is_add_deduction_hard = true;
            }
            '}' => {
                app_state.deductions_state.select_last();
                app_state.is_add_deduction = true;
                app_state.is_add_deduction_hard = false;
            }
            '-' => {
                if let Some(index) = app_state.deductions_state.selected() {
                    app_state.deductions.remove(index);
                }
            }

            _ => {}
        },
        _ => {}
    }

    return false;
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
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

    let deduction_list = List::new(
        app_state
            .deductions
            .iter()
            .map(|x| ListItem::from(format!(" * [{}] -> {}", x.value.clone(), x.desc.clone()))),
    )
    .highlight_style(Style::default().fg(Color::Black).bg(Color::Red).bold());

    frame.render_stateful_widget(deduction_list, inner_area, &mut app_state.deductions_state);

    if app_state.is_add_deduction {
        Paragraph::new(
            app_state.
            input_value.as_str()
        ).render(frame.area(), frame.buffer_mut());
    }
}
