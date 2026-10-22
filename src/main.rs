use std::{thread,time::Duration};
use std::io::{self,Write};
use colored::*;
use rand::RngExt;
use std::fmt::Write as FmtWrite;

const W: usize = 40; // largeur intérieure du cadre

#[derive(Debug,Clone,Copy,PartialEq)]
enum State {
   Init, 
   Normal,
   Alert,
   Error
}

fn top(f: &mut String) {
    writeln!(f, "╔{}╗\x1B[K", "═".repeat(W + 2)).ok();
}

fn sep(f: &mut String) {
    writeln!(f, "╠{}╣\x1B[K", "═".repeat(W + 2)).ok();
}

fn bottom(f: &mut String) {
    writeln!(f, "╚{}╝\x1B[K", "═".repeat(W + 2)).ok();
}

fn title(f: &mut String, text: &str) {
    writeln!(f, "║ {:^W$} ║\x1B[K", text).ok();
}

fn row(f: &mut String, text: &str) {
    writeln!(f, "║ {:<W$} ║\x1B[K", text).ok();
}

fn row_kv(f: &mut String, prefix: &str, plain: &str, shown: ColoredString) {
    let used = prefix.chars().count() + plain.chars().count();
    let pad = W.saturating_sub(used);
    writeln!(f, "║ {}{}{} ║\x1B[K", prefix, shown, " ".repeat(pad)).ok();
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
    let mut temperature: f32 = 20.0;
    let mut state: State = State::Init;
    let mut rng = rand::rng();
    let mut out = io::stdout().lock();

    // une seule fois : on efface l'écran et on cache le curseur
    write!(out, "\x1B[2J\x1B[?25l").ok();

    for tick in 1..=10000 {
        temperature += rng.random_range(-5.0..=5.0);
        temperature = temperature.clamp(0.0, 120.0);

        state = match (state, temperature) {
            (State::Error, t) if t < 40.0 => State::Normal,
            (State::Error, _) => State::Error,
            (_, t) => next_state(t),
        };

        let filled = (temperature.min(100.0) / 100.0 * 20.0) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(20 - filled));
        let bar_colored = match state {
            State::Error => bar.red(),
            State::Alert => bar.yellow(),
            _ => bar.green(),
        };

        // on construit toute l'image dans une String
        let mut frame = String::new();
        frame.push_str("\x1B[H"); // curseur en haut à gauche

        top(&mut frame);
        title(&mut frame, &format!("TICK {:02}", tick));
        sep(&mut frame);
        row(&mut frame, &format!("{:<15} : {:>6.1} °C", "Temperature", temperature));
        row_kv(&mut frame, &format!("{:<15} : ", "State"), label(state), colored_label(state));
        row_kv(&mut frame, &format!("{:<15} : ", "Bar"), &bar, bar_colored);
        row(&mut frame, &format!("{:<15} : {:#010b}", "Brut", temperature as u32));
        bottom(&mut frame);

        frame.push_str("\x1B[J"); // efface ce qui reste en dessous

        // UN SEUL envoi + UN SEUL flush par tick
        write!(out, "{}", frame).ok();
        out.flush().ok();

        thread::sleep(Duration::from_millis(200));
    }

    // on remet le curseur
    write!(out, "\x1B[?25h").ok();
    out.flush().ok();
}
