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
    technique_scores: [Score; 6],
    presentation_scores: [Score; 4],
    current_score_index: u8,

    competitor_name: String,

    technique_state: ListState,
    presentation_state: ListState,

    deductions: Vec<Deduction>,
    deductions_state: ListState,

    is_setup: bool,
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
    tag: String,
    value: f32,
    scored: bool,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_setup = true;
    state.is_add_deduction = false;
    state.current_score_index = 0;

    // DEFAULTS FOR SCORING FREESTYLE
    state.technique_scores = [
        Score {
            tag: String::from("T1"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("T2"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("T3"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("T4"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("T5"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("T6"),
            value: 0.0,
            scored: false
        }
    ];
    state.presentation_scores = [
        Score {
            tag: String::from("P1"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("P2"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("P3"),
            value: 0.0,
            scored: false
        },
        Score {
            tag: String::from("P4"),
            value: 0.0,
            scored: false
        }
    ];

    state.technique_state.select_next();
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
            } else if app_state.is_setup {
                if handle_setup(key, app_state) {
                    break;
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

fn handle_setup(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.competitor_name.push(c);
        }

        event::KeyCode::Backspace => {
            app_state.competitor_name.pop();
        }

        event::KeyCode::Enter => {
            app_state.is_setup = false;
            return false;
        }

        event::KeyCode::Esc => {
            return true;
        }

        _ => {}
    }

    return false;
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

fn register_score(app_state: &mut AppState, score: f32) {

    if app_state.current_score_index < 6 {
        let index = app_state.current_score_index as usize;
        app_state.technique_state.select(Some(index));
        
        if let Some(score_element) = app_state.technique_scores.get_mut(index) {
            score_element.value = score;
            score_element.scored = true;
        }

        if app_state.current_score_index < 5 {
            app_state.technique_state.select_next();
        }
        else {
            app_state.technique_state.select(None);
            app_state.presentation_state.select_next();
        }
    }
    else if app_state.current_score_index < 10 {
        let index = (app_state.current_score_index - 6) as usize;
        app_state.presentation_state.select(Some(index));
        
        if let Some(score_element) = app_state.presentation_scores.get_mut(index) {
            score_element.value = score;
            score_element.scored = true;
        }

        if app_state.current_score_index < 9 {
            app_state.presentation_state.select_next();
        }
    }

    if app_state.current_score_index < 9 {
        app_state.current_score_index += 1;
    }
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
            '|' => { register_score(app_state, 0.0); }
            '1' => { register_score(app_state, 0.1); }
            '2' => { register_score(app_state, 0.2); }
            '3' => { register_score(app_state, 0.3); }
            '4' => { register_score(app_state, 0.4); }
            '5' => { register_score(app_state, 0.5); }
            '6' => { register_score(app_state, 0.6); }
            '7' => { register_score(app_state, 0.7); }
            '8' => { register_score(app_state, 0.8); }
            '9' => { register_score(app_state, 0.9); }
            '0' => { register_score(app_state, 1.0); }

            _ => {}
        },
        _ => {}
    }

    return false;
}

fn score_bar(score: &Score, color: Color) -> Line<'static> {

    // format: T1 -> [X][1][2][3][4][5][6][7][8][9][T]
    let ret = Line::from(vec![
        Span::styled(format!(" {} -> ", score.tag.clone()), Style::default().fg(color).bold()),
        Span::styled("[:X:]", if score.value == 0.0 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:1:]", if score.value == 0.1 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:2:]", if score.value == 0.2 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:3:]", if score.value == 0.3 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:4:]", if score.value == 0.4 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:5:]", if score.value == 0.5 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:6:]", if score.value == 0.6 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:7:]", if score.value == 0.7 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:8:]", if score.value == 0.8 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:9:]", if score.value == 0.9 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) }),
        Span::styled("[:T:]", if score.value == 1.0 && score.scored { Style::default().fg(color).bold() } else { Style::default().fg(Color::Gray) })
    ]);

    return ret
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let main_panel = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
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
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(scoring_panel[0]);

    let technique_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ TECHNIQUE ]]").centered().bold())
        .fg(Color::Green);

    let [technique_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(base_scoring_panel[0]);

    let presentation_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ PRESENTATION ]]").centered().bold())
        .fg(Color::Blue);

    let [presentation_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(base_scoring_panel[1]);

    let deduction_border = Block::bordered()
        .border_type(Double)
        .title_top(Line::from("[[ DEDUCTIONS ]]").centered().bold())
        .fg(Color::Red);

    let [deduction_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(scoring_panel[1]);

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
        let competitor_paragraph = Paragraph::new(format!(" {}", app_state.competitor_name.as_str()))
            .block(competitor_border); // <-- This puts the text INSIDE the box automatically

        frame.render_widget(competitor_paragraph, left_panel[0]);
    }

    let technique_scores = List::new(
        app_state.technique_scores
            .iter()
            .map(|x| ListItem::from(score_bar(x, Color::Green)))
    ).highlight_style(Style::default().bg(Color::DarkGray));

    let presentation_scores = List::new(
        app_state.presentation_scores
            .iter()
            .map(|x| ListItem::from(score_bar(x, Color::Blue)))
    ).highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_stateful_widget(technique_scores, technique_area, &mut app_state.technique_state);
    frame.render_stateful_widget(presentation_scores, presentation_area, &mut app_state.presentation_state);

}
