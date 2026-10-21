use std::{thread,time::Duration};
use std::io::{self,Write};
use colored::*;
use rand::RngExt;

#[derive(Debug,Clone,Copy,PartialEq)]
enum State {
   Init, 
   Normal,
   Alert,
   Error
}

fn clear_screen() {
   print!("\x1B[2J\x1B[H");
   io::stdout().flush().ok();
}

fn label(etat: State) -> &'static str {
  match etat {
    State::Init => "INIT",
    State::Normal => "NORMAL",
    State::Alert => "ALERT",
    State::Error => "ERROR",
  }
}

fn next_state(temp: f32) -> State {
  match temp {
     t if t <  50.0 => State::Normal,
     t if t <= 80.0 => State::Alert,
     _ => State::Error,
  }
}

fn colored_label(state: State) -> ColoredString {
  match state {
     State::Init => label(state).white().bold(),
     State::Normal => label(state).green().bold(),
     State::Alert => label(state).yellow().bold(),
     State::Error => label(state).red().bold(),
  }
}

fn main() {
   let mut temperature : f32 = 20.0;
   let mut state : State = State::Init;
   let settings_width = 15;
   let mut rng = rand::rng();
  
  for tick in 1..=10000 {
     clear_screen();
     temperature += rng.random_range(-5.0..=5.0);
     temperature = temperature.clamp(0.0, 120.0);     

     state = match (state,temperature) {
        (State::Error, t) if t < 40.0 => State::Normal,
        (State::Error,_) => State::Error,
        (_,t) => next_state(t),
     };     
 
      let filled = (temperature.min(100.0) / 100.0 * 20.0) as usize;
      let bar = format!("{}{}","█".repeat(filled),"░".repeat(20-filled));
      let bar_colored = match state {
          State::Error => bar.red(),
          State::Alert => bar.yellow(),
          _ => bar.green(),
      };
     
      println!("{:-^40}",format!(" TICK {:02} ",tick));
      println!("{:<settings_width$} : {} °C","Temperature",temperature);
      println!("{:<settings_width$} : {}","State",colored_label(state));
      println!("{:<settings_width$} : {}","Bar",bar_colored);
      println!("{:<settings_width$} : {:#010b}","Brut", temperature as u32);
      
      thread::sleep(Duration::from_millis(200));
   }
}
