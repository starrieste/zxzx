use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io::{self, Write};
use std::time::{Instant, Duration};

fn main() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    println!("Start Tapping!");

    let mut history: Vec<Instant> = Vec::new();
    let mut total_taps: u64 = 0;
    let mut last = None;
    
    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                    break;
                }
                
                if Some(key_event.code) == last {
                    continue;
                }

                total_taps += 1;
                history.push(Instant::now());

                last = Some(key_event.code);
            }
        }

        let now = Instant::now();
        history.retain(|&t| now.duration_since(t) < Duration::from_secs(1));

        let bpm = if history.len() > 1 {
            let first = history[0];
            let last = history[history.len() - 1];
            let actual_duration = last.duration_since(first).as_secs_f64();
            
            if actual_duration > 0.0 {
                (history.len() as f64 / actual_duration) * 60.0 / 4.0
            } else { 0.0 }
        } else {
            0.0
        };

        print!("\x1B[1A\x1B[2K\rTotal Taps: {} | BPM: {:.2}\r\n", total_taps, bpm);
        io::stdout().flush()?;
    }
    
    crossterm::terminal::disable_raw_mode()?;
    println!("\nsee you next time!");

    Ok(())
}
