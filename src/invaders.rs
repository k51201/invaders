use std::{time::Duration, cmp::max};

use rusty_time::Timer;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}};

pub struct Invader {
  x: usize,
  y: usize,
}

pub struct Invaders {
  pub fleet: Vec<Invader>,
  move_timer: Timer,
  direction: i32,
}

impl Invaders {
  pub fn new() -> Self {
    let mut fleet = Vec::new();
    for x in 0..NUM_COLS {
      for y in 0..NUM_ROWS {
        if (1 < x) && (x < NUM_COLS - 2)
          && (0 < y) && (y < 9)
          && (x % 2 == 0) && (y % 2 == 0) {
            fleet.push(Invader { x, y });
          }
      }
    }

    Self { fleet, move_timer: Timer::from_millis(2000), direction: 1 }
  }

  pub fn update(&mut self, delta: Duration) -> bool {
    self.move_timer.update(delta);
    if self.move_timer.ready {
      self.move_timer.reset();
      let mut downwards = false;
      if self.direction == -1 {
        let min_x = self.fleet.iter().map(|inv| inv.x).min().unwrap_or(0);
        if min_x == 0 {
          self.direction = 1;
          downwards = true;
        }
      } else {
        let max_x = self.fleet.iter().map(|inv| inv.x).max().unwrap_or(0);
        if max_x == NUM_COLS - 1 {
          self.direction = -1;
          downwards = true;
        }
      }
      if downwards {
        let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
        self.move_timer = Timer:: from_millis(new_duration as u64);
        for inv in self.fleet.iter_mut() {
          inv.y += 1;
        }
      } else {
        for inv in self.fleet.iter_mut() {
          inv.x = ((inv.x as i32) + self.direction) as usize;
        }
      }
      true
    } else {
      false
    }
  }
}

impl Drawable for Invaders {
  fn draw(&self, frame: &mut Frame) {
      for inv in self.fleet.iter() {
        let time_left = self.move_timer.time_left.as_secs_f32();
        let duration = self.move_timer.duration.as_secs_f32();
        frame[inv.x][inv.y] = if (time_left / duration) > 0.5 {"x"} else {"+"};
      }
  }
}
