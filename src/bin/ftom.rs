use clap::Parser;

#[derive(Parser)]
#[command(
    name = "ftom",
    about = "Convert frequency to MIDI note number and note name",
    version
)]
struct Args {
    /// Frequency in Hz (e.g., 440, 442.0)
    #[arg(value_parser = parse_frequency)]
    frequency: f64,
}

fn parse_frequency(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| String::from("Invalid frequency value"))
        .and_then(|f| {
            if f > 0.0 {
                Ok(f)
            } else {
                Err(String::from("Frequency must be greater than 0"))
            }
        })
}

fn freq_to_midi(freq: f64) -> f64 {
    // Inverse of MIDI to frequency formula:
    // n = 69 + 12 * log2(f/440)
    69.0 + 12.0 * (freq / 440.0).log2()
}

fn get_note_name(midi_note: u8) -> String {
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note_index = midi_note as usize % 12;
    let octave = (midi_note as i32 / 12) - 1;
    format!("{}{}", note_names[note_index], octave)
}

fn main() {
    let args = Args::parse();
    let midi_float = freq_to_midi(args.frequency);
    let midi_note = midi_float.round() as u8;

    if midi_note > 127 {
        eprintln!("Error: Frequency too high for MIDI range");
        std::process::exit(1);
    }

    let cents = (midi_float - midi_float.round()) * 100.0;
    let note_name = get_note_name(midi_note);

    println!("Frequency: {:.2} Hz", args.frequency);
    println!("MIDI note: {} ({})", midi_note, note_name);
    if cents.abs() > 0.01 {
        println!("Cents: {:+.1}", cents);
    }
}
