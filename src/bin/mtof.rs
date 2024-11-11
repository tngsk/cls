use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[command(
    name = "mtof",
    about = "Convert MIDI note number or note name to frequency",
    version
)]
struct Args {
    /// MIDI note number (0-127) or note name (e.g., C4, G#4, Eb5)
    note: String,
}

#[derive(Debug)]
struct NoteParseError;

struct Note(u8);

impl FromStr for Note {
    type Err = NoteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as MIDI note number first
        if let Ok(midi_note) = s.parse::<u8>() {
            if midi_note <= 127 {
                return Ok(Note(midi_note));
            }
        }

        // Parse as note name
        let s = s.trim();
        if s.len() < 2 {
            return Err(NoteParseError);
        }

        let (note_part, octave_str) = s.split_at(s.len() - 1);
        let octave = octave_str.parse::<i32>().map_err(|_| NoteParseError)?;

        let (base_note, semitones) = match note_part.to_uppercase().as_str() {
            "C" => (0, 0),
            "C#" | "DB" => (0, 1),
            "D" => (2, 0),
            "D#" | "EB" => (2, 1),
            "E" => (4, 0),
            "F" => (5, 0),
            "F#" | "GB" => (5, 1),
            "G" => (7, 0),
            "G#" | "AB" => (7, 1),
            "A" => (9, 0),
            "A#" | "BB" => (9, 1),
            "B" => (11, 0),
            _ => return Err(NoteParseError),
        };

        let midi_note = (octave + 1) * 12 + base_note + semitones;
        if midi_note >= 0 && midi_note <= 127 {
            Ok(Note(midi_note as u8))
        } else {
            Err(NoteParseError)
        }
    }
}

fn midi_to_freq(midi_note: u8) -> f64 {
    // MIDI note number to frequency conversion formula:
    // f = 440 * 2^((n-69)/12)
    // where n is the MIDI note number and 440Hz is A4 (MIDI note 69)
    440.0 * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0)
}

fn main() {
    let args = Args::parse();

    match args.note.parse::<Note>() {
        Ok(Note(midi_note)) => {
            let freq = midi_to_freq(midi_note);
            println!("{:.2} Hz (MIDI note: {})", freq, midi_note);
        }
        Err(_) => {
            eprintln!("Error: Invalid note format. Use MIDI note number (0-127) or note name (e.g., C4, D#5, Eb6)");
            std::process::exit(1);
        }
    }
}
