use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
};
use tui_big_text::{BigText, PixelSize};

struct State {
    page: i64,
}

fn main() -> color_eyre::Result<()> {
    let mut state = State { page: 1 };

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
                    terminal.draw(|frame| render_game(frame))?;
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
    let temp_text = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![start.into()])
        .build();
    frame.render_widget(temp_text, area);
}

fn render_game(frame: &mut Frame) {
    let area = frame.area();
}
