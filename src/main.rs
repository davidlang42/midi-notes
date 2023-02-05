use std::collections::HashMap;
use std::io::Write;
use std::{env, fs};
use wmidi::{MidiMessage, Note, U7};
use console::Term;

fn main() {
    let mut args = env::args().skip(1);
    let midi_out: String = args.next().expect("The first argument should be the MIDI output device (or - for std out).");
    let all_notes = list_all_notes();
    let notes: Vec<Note> = args.map(|a| all_notes.get(&a).expect(&format!("Invalid note: {}", a)).clone()).collect();
    let mut f: Box<dyn Write> = if midi_out == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(fs::File::options().write(true).open(&midi_out).expect(&format!("Cannot open MIDI OUT '{}'", midi_out)))
    };
    let names: Vec<String> = notes.iter().map(|n| format!("{}", n)).collect();
    println!("Sending notes: {}", names.join(", "));
    send_notes(&mut f, &notes, true);
    println!("Press any key to stop.");
    let _ = Term::stdout().read_key();
    send_notes(&mut f, &notes, false);
}

fn send_notes(f: &mut Box<dyn Write>, notes: &Vec<Note>, state: bool) {
    for note in notes {
        let message = if state {
            MidiMessage::NoteOn(wmidi::Channel::Ch1, note.clone(), U7::from_u8_lossy(100))
        } else {
            MidiMessage::NoteOff(wmidi::Channel::Ch1, note.clone(), U7::from_u8_lossy(0))
        };
        let mut buf = Vec::new();
        let expected = message.bytes_size();
        buf.resize(expected, 0);
        if message.copy_to_slice(&mut buf).is_err() {
            panic!("Error writing MIDI message");
        }
        if f.write_all(&buf).is_err() {
            panic!("Error writing to device.")
        }
        if f.flush().is_err() {
            panic!("Error flushing to device.");
        }
    }
}

fn list_all_notes() -> HashMap<String, Note> {
    let mut map = HashMap::new();
    //TODO make this handle enharmonics (eg. C# rather than Db)
    //TODO make this handle negative octaves (eg. C#-1)
    for i in 0..127 {
        let note = Note::try_from(i).unwrap();
        let s = format!("{}", note);
        map.insert(s, note);
    }
    map
}