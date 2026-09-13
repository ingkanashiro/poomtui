use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame, crossterm::{
        event::{self, Event, KeyEvent},
        terminal,
    }, layout::{
        Constraint,
        Direction::{self, Horizontal},
        Layout,
    }, style::{Color, Style, Stylize}, symbols::border::{DOUBLE, THICK}, text::{Line, Span}, widgets::{
        Block, BorderType::{self, Double, Thick}, List, ListItem, ListState, Paragraph, Widget,
    },
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
    Escape,
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
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_add_deduction = false;

                        if app_state.is_add_deduction_hard {
                            app_state.deductions.push(Deduction {
                                value: 0.1,
                                desc: app_state.input_value.clone(),
                            })
                        } else {
                            app_state.deductions.push(Deduction {
                                value: 0.3,
                                desc: app_state.input_value.clone(),
                            })
                        }

                        app_state.input_value.clear();
                    }
                    FormAction::Escape => {
                        app_state.is_add_deduction = false;
                        app_state.input_value.clear();
                    }
                }
            } else {
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

fn score_bar(score: f32, color: Color) -> Line<'static> {

    // format: [X][1][2][3][4][5][6][7][8][9][T]
    let ret = Line::from(vec![
        Span::styled("[X]", if (score == 0.0) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[1]", if (score == 0.1) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[2]", if (score == 0.2) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[3]", if (score == 0.3) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[4]", if (score == 0.4) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[5]", if (score == 0.5) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[6]", if (score == 0.6) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[7]", if (score == 0.7) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[8]", if (score == 0.8) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[9]", if (score == 0.9) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() }),
        Span::styled("[T]", if (score == 1.0) { Style::default().fg(color) } else { Style::default().fg(Color::Gray).bold() })
    ]);

    return ret
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let main_panel = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(frame.area());

    let left_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(main_panel[0]);

    let rank_panel_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ RANKING ]]").centered().bold())
        .fg(Color::Yellow);

    let competitor_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from(" Contestant: ").bold())
        .fg(Color::White);

    let deduction_field_border = Block::bordered()
        .border_type(Thick)
        .title_top(Line::from(" Adding new deduction... ").bold())
        .fg(Color::LightRed);

    let [deduction_field_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_panel[0]);

    let scoring_panel = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(left_panel[1]);

    let base_scoring_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(scoring_panel[0]);

    let technique_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ TECHNIQUE ]]").centered().bold())
        .fg(Color::Green);

    let presentation_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ PRESENTATION ]]").centered().bold())
        .fg(Color::Blue);

    let deduction_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ DEDUCTIONS ]]").centered().bold())
        .fg(Color::Red);

    let [deduction_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(scoring_panel[1]);

    // let competitor_area = Paragraph::default()
    //     .block(competitor_border);

    /*
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());



    Block::bordered()
        .border_type(Thick)
        .fg(Color::Red)
        .title_top(Line::from("[ Deductions ]").centered().bold())
        .render(border_area, frame.buffer_mut());
    */


    let deduction_list = List::new(
        app_state
            .deductions
            .iter()
            .map(|x| ListItem::from(format!(" * [{}] -> {}", x.value.clone(), x.desc.clone()))),
    )
    .highlight_style(Style::default().fg(Color::Black).bg(Color::Red).bold());

    frame.render_widget(rank_panel_border, main_panel[1]);
    frame.render_widget(technique_border, base_scoring_panel[0]);
    frame.render_widget(presentation_border, base_scoring_panel[1]);
    frame.render_widget(deduction_border, scoring_panel[1]);

    frame.render_stateful_widget(deduction_list, deduction_area, &mut app_state.deductions_state);

    if app_state.is_add_deduction {
        let deduction_paragraph = Paragraph::new(format!(" {}", app_state.input_value.as_str()))
            .block(deduction_field_border); // <-- This puts the text INSIDE the box automatically

        frame.render_widget(deduction_paragraph, left_panel[0]);
    }
    else {
        frame.render_widget(competitor_border, left_panel[0]);
    }

}
