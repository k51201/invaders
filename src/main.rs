use std::{error::Error, io, time::Duration, sync::mpsc, thread};
use crossterm::{terminal, ExecutableCommand, cursor, event::{self, Event, KeyCode}};
use invaders::{frame, render};
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
  
  // render loop
  let (render_tx, render_rx) = mpsc::channel();
  let render_handle = thread::spawn( move || {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    loop {
      let curr_frame = match render_rx.recv() {
        Ok(x) => x,
        Err(_) => break,
      };
      render::render(&mut stdout, &last_frame, &curr_frame, false);
    }
  });

  // game loop
  'gameloop: loop {
    let curr_frame = frame::new_frame();

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

    let _ = render_tx.send(curr_frame);
    thread::sleep(Duration::from_millis(1));
  }
  
  // cleanup
  drop(render_tx);
  render_handle.join().unwrap();
  audio.wait();
  stdout.execute(terminal::LeaveAlternateScreen)?;
  stdout.execute(cursor::Show)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
