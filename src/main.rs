use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use std::io;
use tokio::{
    select, task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw)?;

        // Use `spawn_blocking` to avoid blocking the async runtime
        let event = task::spawn_blocking(event::read).await??;

        if matches!(event, Event::Key(_)) {
            break;
        }

        // Optional: sleep to avoid tight loop
        sleep(Duration::from_millis(50)).await;
    }

    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title bar"), title_area);
    frame.render_widget(Block::bordered().title("Status bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
