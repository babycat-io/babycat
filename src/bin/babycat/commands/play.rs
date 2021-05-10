// This whole file is very poorly written. It will work for now, but we need to rewrite it.

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize};

use rodio::Sample;
use rodio::Source;

use babycat::{FloatWaveform, Waveform};

const NANOSECONDS_PER_SECOND: u64 = 1000000000;

const THIRTY_MILLISECONDS: std::time::Duration = std::time::Duration::from_millis(30);

struct FloatWaveformSource {
    frame_rate_hz: u32,
    num_channels: u16,
    num_frames: usize,
    num_samples: usize,
    current_sample_idx: usize,
    duration: std::time::Duration,
    interleaved_samples: Vec<f32>,
}

impl From<&FloatWaveform> for FloatWaveformSource {
    fn from(item: &FloatWaveform) -> Self {
        let frame_rate_hz = item.frame_rate_hz();
        let num_channels = item.num_channels() as u16;
        let num_frames = item.num_frames() as usize;
        let num_samples = (num_channels as u64 * item.num_frames()) as usize;
        let current_sample_idx: usize = 0;
        let duration_seconds_part = item.num_frames() / (frame_rate_hz as u64);
        let duration_nanoseconds_part = ((item.num_frames() % frame_rate_hz as u64)
            * NANOSECONDS_PER_SECOND
            / (frame_rate_hz as u64)) as u32;
        let duration = std::time::Duration::new(duration_seconds_part, duration_nanoseconds_part);
        Self {
            frame_rate_hz,
            num_channels,
            num_frames,
            num_samples,
            current_sample_idx,
            duration,
            interleaved_samples: item.interleaved_samples().to_vec(),
        }
    }
}

impl Iterator for FloatWaveformSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current_sample_idx < self.num_samples {
            let val = self.interleaved_samples[self.current_sample_idx];
            self.current_sample_idx += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl rodio::source::Source for FloatWaveformSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.num_frames)
    }

    fn channels(&self) -> u16 {
        self.num_channels
    }

    fn sample_rate(&self) -> u32 {
        self.frame_rate_hz
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(self.duration)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PlayStatus {
    Stopped,
    Playing,
    Paused,
}

impl std::fmt::Display for PlayStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PlayStatus::Stopped => write!(f, "Stopped"),
            PlayStatus::Playing => write!(f, "Playing"),
            PlayStatus::Paused => write!(f, "Paused"),
        }
    }
}

struct AudioPlayer {
    timestamp_at_play_start: std::time::Instant,
    duration_at_play_start: std::time::Duration,
    duration_at_pause_start: std::time::Duration,
    source_duration: Option<std::time::Duration>,
    play_status: PlayStatus,
    volume: i16,
    sink: rodio::Sink,
}

impl AudioPlayer {
    fn new<R: rodio::source::Source + 'static>(sink: rodio::Sink, source: R) -> Self
    where
        <R as Iterator>::Item: Sample,
        R: Send,
        <R as Iterator>::Item: Send,
    {
        let source_duration = source.total_duration();
        sink.pause();
        sink.append(source);
        Self {
            timestamp_at_play_start: std::time::Instant::now(),
            duration_at_play_start: std::time::Duration::new(0, 0),
            duration_at_pause_start: std::time::Duration::new(0, 0),
            source_duration,
            play_status: PlayStatus::Paused,
            volume: 100,
            sink,
        }
    }

    fn play_status(&self) -> PlayStatus {
        self.play_status
    }

    fn play(&mut self) {
        self.duration_at_play_start = std::time::Duration::new(0, 0);
        self.timestamp_at_play_start = std::time::Instant::now();
        self.sink.play();
        self.play_status = PlayStatus::Playing;
    }

    fn pause(&mut self) {
        self.duration_at_pause_start = self.current_play_duration();
        self.sink.pause();
        self.play_status = PlayStatus::Paused;
    }

    fn unpause(&mut self) {
        self.timestamp_at_play_start = std::time::Instant::now();
        self.duration_at_play_start = self.duration_at_pause_start;
        self.sink.play();
        self.play_status = PlayStatus::Playing;
    }

    fn toggle_play_pause(&mut self) {
        match self.play_status {
            PlayStatus::Stopped => self.play(),
            PlayStatus::Playing => self.pause(),
            PlayStatus::Paused => self.unpause(),
        }
    }

    fn stop(&mut self) {
        self.duration_at_play_start = std::time::Duration::new(0, 0);
        self.sink.stop();
        self.play_status = PlayStatus::Stopped;
    }

    fn current_play_duration(&mut self) -> std::time::Duration {
        let calculated_duration = if self.play_status == PlayStatus::Playing {
            let duration_since_play_start =
                std::time::Instant::now() - self.timestamp_at_play_start;
            duration_since_play_start + self.duration_at_play_start
        } else {
            self.duration_at_pause_start
        };
        if let Some(source_duration) = self.source_duration {
            std::cmp::min(calculated_duration, source_duration)
        } else {
            calculated_duration
        }
    }

    fn set_volume(&mut self, volume: i16) {
        let clamped_volume = if volume > 200 {
            200
        } else if volume < 1 {
            0
        } else {
            volume
        };
        self.sink.set_volume(clamped_volume as f32 / 100.0);
        self.volume = clamped_volume;
    }

    fn increase_volume(&mut self) {
        self.set_volume(self.volume + 5)
    }

    fn decrease_volume(&mut self) {
        self.set_volume(self.volume - 5)
    }

    fn volume(&self) -> i16 {
        self.volume
    }
}

fn ui_loop(filename: String) {
    let mut stdout = std::io::stdout();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let waveform = FloatWaveform::from_file(&filename, Default::default()).unwrap();
    let source = FloatWaveformSource::from(&waveform);
    let source_duration = source.total_duration().unwrap();
    let mut audio_player = AudioPlayer::new(sink, source);

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || loop {
        tx.send(read().unwrap()).unwrap();
    });

    loop {
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(format!("File: {}", filename)),
            MoveTo(0, 1),
            Print(format!(
                "Status: {}",
                audio_player.play_status().to_string()
            )),
            MoveTo(40, 1),
            Print(format!("Volume: {}%", audio_player.volume())),
            MoveTo(0, 3),
            Print(format!("{:?}", audio_player.current_play_duration())),
            MoveTo(40, 3),
            Print(format!("{:?}", source_duration)),
            MoveTo(0, 6),
            Print("space - Play/Pause\tUp/Down - Volume\tq - Quit"),
        )
        .unwrap();

        match rx.try_recv() {
            Ok(event) => match event {
                // Toggle between play/pause on the space key.
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                }) => audio_player.toggle_play_pause(),
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                }) => audio_player.stop(),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                }) => audio_player.decrease_volume(),
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                }) => audio_player.increase_volume(),
                // Quit on Ctrl-C.
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    break;
                }
                // Quit on the 'q' key.
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    break;
                }
                _ => (),
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => (),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
        std::thread::sleep(THIRTY_MILLISECONDS);
    }
}

pub fn play(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().unwrap();
    execute!(
        std::io::stdout(),
        SetSize(80, 25),
        MoveTo(0, 0),
        Clear(ClearType::All),
        Hide,
    )
    .unwrap();
    ui_loop(filename);
    execute!(std::io::stdout(), MoveTo(0, 0), Clear(ClearType::All), Show,).unwrap();
    disable_raw_mode().unwrap();
    Ok(())
}
