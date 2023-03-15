use std::{error::Error, io, time::Duration};
use crossterm::{terminal, ExecutableCommand, cursor, event::{self, Event, KeyCode}};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
  // audio
  let mut audio = Audio::new();
  audio.add("explode", "explode.wav");
  audio.add("lose", "lose.wav");
  audio.add("move", "move.wav");
  audio.add("pew", "pew.wav");
  audio.add("startup", "startup.wav");
  audio.add("win", "win.wav");

  audio.play("startup");

  // terminal
  let mut stdout = io::stdout();
  terminal::enable_raw_mode()?;
  stdout.execute(terminal::EnterAlternateScreen)?;
  stdout.execute(cursor::Hide)?;
  
  // game loop
  'gameloop: loop {
    while event::poll(Duration::default())?  {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
              audio.play("lose");
              break 'gameloop;
            }
            _ => {}
        }
      }
    }
  }
  
  // cleanup
  audio.wait();
  stdout.execute(terminal::LeaveAlternateScreen)?;
  stdout.execute(cursor::Show)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
