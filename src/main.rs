use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

struct State {
    page: i64,
}

struct Pong {
    x_ball: f64,
    y_ball: f64,
    vx_ball: f64,
    vy_ball: f64,
    one_paddle: f64,
    two_paddle: f64,
    one_score: i64,
    two_score: i64,
}

impl Pong {
    fn update(&mut self, width: f64, height: f64) {}
}

fn main() -> color_eyre::Result<()> {
    let mut state = State { page: 1 };
    let mut game = Pong {
        x_ball: 10.0,
        y_ball: 10.0,
        vx_ball: 1.0,
        vy_ball: 1.0,
        one_paddle: 5.0,
        two_paddle: 5.0,
        one_score: 0,
        two_score: 0,
    };

    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            match state.page {
                1 => {
                    terminal.draw(|frame| render(frame))?;
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Enter => state.page = 2,
                            KeyCode::Esc => break Ok(()),
                            _ => {}
                        }
                    }
                }
                _ => {
                    terminal.draw(|frame| render_game(frame, &game))?;
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Esc => break Ok(()),
                            _ => {}
                        }
                    }
                }
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let area = frame.area();
    let start = "Press Enter To Start Game";
    let start_text = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![start.into()])
        .build();
    frame.render_widget(start_text, area);
}

fn render_game(frame: &mut Frame, pong: &Pong) {
    let area = frame.area();
    let split = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
    ]);

    let [time, game, score, names] = split.areas(area);

    let clock = "00:00";
    let time_text = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![clock.into()])
        .build();
    frame.render_widget(time_text, time);

    let one_paddle_rect = Rect::new(1, pong.one_paddle as u16, 1, 3);
    frame.render_widget(Block::default().bg(Color::White), one_paddle_rect);

    let ball_rect = Rect::new(pong.x_ball as u16, pong.y_ball as u16, 1, 1);
    frame.render_widget(Paragraph::new("●").fg(Color::Yellow), ball_rect);

    let two_paddle_rect = Rect::new((game.width - 1) as u16, pong.two_paddle as u16, 1, 3);
    frame.render_widget(Block::default().bg(Color::White), two_paddle_rect);

    let score_split = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [score_one, score_two] = score_split.areas(score);
    frame.render_widget(Paragraph::new("0").centered().bold(), score_one);
    frame.render_widget(Paragraph::new("0").centered().bold(), score_two);
    let name_split = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [name_one, name_two] = name_split.areas(names);
    frame.render_widget(Paragraph::new("Player 1").centered().bold(), name_one);
    frame.render_widget(Paragraph::new("Player 2").centered().bold(), name_two);
}
