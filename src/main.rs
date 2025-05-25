use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    DefaultTerminal, Frame,
};
use std::io;
use tokio::{
    select, task,
    time::{sleep, Duration},
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    actions: Actions,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.actions.run();
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        frame.render_widget(Block::bordered().title("Pinyin UI"), title_area);
        frame.render_widget(Block::bordered().title("'q':Quit, 'r':Record"), status_area);
        frame.render_widget(Block::bordered().title("Left"), left_area);
        frame.render_widget(Block::bordered().title("Right"), right_area);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('j') | KeyCode::Down => self.should_quit = true,
                KeyCode::Char('k') | KeyCode::Up => self.should_quit = true,
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Actions {}

impl Actions {
    fn run(&self) {
        // clone this actions to pass to background task
        let this = self.clone();
    }
}
