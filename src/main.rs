use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    DefaultTerminal, Frame,
};
use tokio::time::Duration;
use tokio_stream::StreamExt;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

/// Starts recording and saves to a WAV-like buffer (or stream it elsewhere)
pub async fn start_audio_stream(recording: Arc<Mutex<bool>>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device.default_input_config()?;

    let sample_format = config.sample_format();
    let config = cpal::StreamConfig {
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };

    let err_fn = |err| eprintln!("Stream error: {}", err);

    let recording_clone = recording.clone();
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _| {
                if *recording_clone.lock().unwrap() {
                    // Process audio samples
                    println!("Recording {} samples", data.len());
                    // You can collect into a buffer or stream to an AI service
                }
            },
            err_fn,
            None,
        )?,
        _ => unimplemented!("Unsupported format"),
    };

    stream.play()?;
    println!("Audio stream started");

    // Keep the stream alive while recording
    tokio::spawn(async move {
        while *recording.lock().unwrap() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        println!("Audio stream stopped");
    });

    Ok(())
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
                KeyCode::Char('r') | KeyCode::Enter => {
                    let mut actions = self.actions.clone();
                    tokio::spawn(async move {
                        actions.toggle_recording().await;
                    });
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Actions {
    recording: bool,
}

impl Actions {
    fn run(&self) {
        // clone this actions to pass to background task
        let this = self.clone();
    }

    pub async fn toggle_recording(&mut self) {
        let shared_flag = Arc::new(Mutex::new(true));
        if self.recording {
            *shared_flag.lock().unwrap() = false;
            self.recording = false;
            println!("Recording stopped");
        } else {
            self.recording = true;
            let flag_clone = Arc::clone(&shared_flag);
            tokio::spawn(async move {
                let _ = start_audio_stream(flag_clone).await;
            });
            println!("Recording started");
        }
    }
}
